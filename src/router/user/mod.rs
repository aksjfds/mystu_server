use glacier::prelude::*;

use serde::{Deserialize, Serialize};

mod login;
mod refresh;
mod sign_up;
mod verify_email;

use login::login;
use refresh::refresh;
use sign_up::sign_up;
use verify_email::verify_email;

use crate::middles::{get, post};
pub async fn route(mut req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    match req.next_route().as_deref() {
        Some("/login") => req.filter(post)?.async_apply(login).await,
        Some("/sign_up") => req.filter(post)?.async_apply(sign_up).await,
        Some("/refresh") => req.filter(get)?.async_apply(refresh).await,
        Some("/verify_email") => req.filter(get)?.async_apply(verify_email).await,
        _ => Ok(HttpResponse::Ok().plain("404")),
    }
}

pub(super) const REFRESH_KEY: &[u8] = b"MYSTU_REFRESH_KEY";
pub(super) const ACCESS_KEY: &[u8] = b"MYSTU_ACCESS_KEY";
pub(super) const REFRESH_DURATION: chrono::TimeDelta = chrono::Duration::days(7);
pub(super) const ACCESS_DURATION: chrono::TimeDelta = chrono::Duration::hours(1);

#[derive(Serialize, Deserialize)]
pub struct Claims<T> {
    pub payload: T,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoginPayload {
    pub email: String,
    pub username: String,
    pub role: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub(self) struct SignUpClaims<T> {
    pub email: T,
    pub verify_code: T,
}

pub fn generate_token<T: Serialize>(
    payload: &T,
    secret: &[u8],
    duration: chrono::Duration,
) -> Result<String, ()> {
    use chrono::Utc;
    use jsonwebtoken::{self, EncodingKey, Header};

    jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            payload: payload,
            exp: (Utc::now() + duration).timestamp() as usize,
        },
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| tracing::debug!("Error when generate token: {}", e))
}

pub fn verify_token<T: for<'a> Deserialize<'a>>(token: &HeaderValue, key: &[u8]) -> Result<T, ()> {
    use jsonwebtoken::{DecodingKey, Validation};

    let token = token.to_str().map_err(|e| tracing::debug!("{:#?}", e))?;

    // 验证jwt
    let mut validation = Validation::default();
    validation.validate_exp = true;
    jsonwebtoken::decode::<Claims<T>>(&token, &DecodingKey::from_secret(key), &validation)
        .map(|token| token.claims.payload)
        .map_err(|e| tracing::debug!("Error when decode verify token: {}", e))
}
