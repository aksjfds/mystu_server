use glacier::prelude::*;
use jsonwebtoken::{DecodingKey, Validation};
use tracing::instrument;

use crate::sql::MyPool;

use super::{ACCESS_DURATION, ACCESS_KEY, LoginPayload, REFRESH_DURATION, REFRESH_KEY};

use super::login::Token;
use super::{Claims, generate_token};

// 通过长token获取短token
#[instrument(level = "debug", skip(req))]
pub(super) async fn refresh(req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    let res = HttpResponse::Ok();

    // 获取长token
    let refresh_token = req
        .headers()
        .get(AUTHORIZATION)
        .map(AsRef::as_ref)
        .map(std::str::from_utf8)
        .ok_or_else(|| tracing::debug!("Authorization is none"))?
        .map_err(|e| tracing::debug!("{}", e))?;

    // 验证长token
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let claims = jsonwebtoken::decode::<Claims<LoginPayload>>(
        &refresh_token,
        &DecodingKey::from_secret(REFRESH_KEY),
        &validation,
    )
    .map(|token| token.claims)
    .map_err(|e| tracing::debug!("Error when decode refresh_token: {}", e))?;
    tracing::info!("this guy try to refresh token: {:#?}", claims.payload);

    // 生成键名，在redis中判断是否存在
    let len = refresh_token.len();
    let status = match len > 16 {
        true => &refresh_token[len - 16..],
        false => refresh_token,
    };
    MyPool::redis_get_del::<_, Option<u8>>(status)
        .map_err(|e| tracing::debug!("{:#?}", e))?
        .ok_or(())?;

    // 存在的话，生成长短token
    let payload = claims.payload;
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
