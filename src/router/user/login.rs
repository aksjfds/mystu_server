use glacier::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::generate_token;
use super::{ACCESS_DURATION, ACCESS_KEY, LoginPayload, REFRESH_DURATION, REFRESH_KEY};
use crate::sql::MyPool;

// 登录成功的发送长短token,长token加入到redis
#[instrument(level = "debug", skip(req))]
pub(super) async fn login(mut req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    let res = HttpResponse::Ok();

    // 获取邮箱和密码
    let body = req
        .body_mut()
        .data()
        .await
        .ok_or_else(|| tracing::debug!("Body is None!"))?
        .map_err(|e| tracing::debug!("{}", e))?;

    let user = serde_json::from_slice::<User<&str>>(&body)
        .map_err(|e| tracing::debug!("Error when parsing body: {}", e))?;

    // 判断是否存在该用户
    let pool = MyPool::new();
    let payload:LoginPayload = sqlx::query_as(
        "SELECT email, username, role FROM users WHERE email = $1 AND active = TRUE AND password = $2",
    )
    .bind(user.email)
    .bind(user.password)
    .fetch_optional(pool)
    .await
    .map_err(|e| tracing::debug!("{}", e))?
    .ok_or_else(|| tracing::debug!("this guy is not exists: {:#?}", user))?;
    tracing::info!("this guy try to login: {:#?}", payload);

    // 生成长短token,保存在 localStorage
    let refresh_token = generate_token(&payload, REFRESH_KEY, REFRESH_DURATION)?;
    let access_token = generate_token(&payload, ACCESS_KEY, ACCESS_DURATION)?;

    // 根据长token生成一个键名存在redis（不需要值也行）
    let len = refresh_token.len();
    let status = match len > 16 {
        true => &refresh_token[len - 16..],
        false => &refresh_token,
    };
    MyPool::redis_set_ex(status, 0u8, REFRESH_DURATION).map_err(|e| tracing::debug!("{:#?}", e))?;

    // 返回长短token
    Ok(res.json(Token {
        refresh_token,
        access_token,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct User<T> {
    pub email: T,
    pub password: T,
}

#[derive(Serialize)]
pub(super) struct Token<T> {
    pub refresh_token: T,
    pub access_token: T,
}

#[tokio::test]
async fn test() {
    let pool = MyPool::new();
    let exists: Option<LoginPayload> = sqlx::query_as(
        "SELECT email, username, role FROM users WHERE email = $1 AND active = TRUE AND password = $2",
    )
    .bind("22qyli13@stu.edu.cn")
    .bind("123456")
    .fetch_optional(pool)
    .await
    .unwrap();

    println!("{:#?}", exists);
}
