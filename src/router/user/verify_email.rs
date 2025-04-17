use glacier::prelude::*;

use chrono::Duration;
use tracing::instrument;

use crate::prelude::param::*;
use crate::router::user::generate_token;
use crate::sql::MyPool;
use crate::tool::{random_verify_code, stu};

use super::SignUpClaims;
use super::sign_up::SIGN_UP_KEY;

#[instrument(level = "debug", skip(req))]
pub(super) async fn verify_email(req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    // 成功则返回1, 否则返回0
    let res = HttpResponse::Ok();

    // 获取参数
    let param: Email = req.param()?;
    tracing::info!("{:#?}", param);

    // 检查该邮箱是否被注册
    let pool = MyPool::new();
    let exists = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
        .bind(param.email.as_str())
        .fetch_one(pool)
        .await
        .map_err(|e| tracing::debug!("{}", e))?;

    if exists {
        tracing::debug!("Email is already Exists!");
        Err(())?
    }

    // 生成随机验证码
    let verify_code = random_verify_code();

    // 生成 JWT
    let payload = SignUpClaims {
        email: param.email.as_str(),
        verify_code: verify_code.as_str(),
    };

    let jwt = generate_token(&payload, SIGN_UP_KEY, Duration::minutes(5))?;

    // 发送邮件验证码
    stu(&param.email, verify_code).map_err(|e| tracing::debug!("Error when send Email: {}", e))?;

    Ok(res.easy_json(jwt))
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub email: String,
}
