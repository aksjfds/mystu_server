use glacier::prelude::*;
use tracing::instrument;

use crate::prelude::param::*;
use crate::prelude::sql::*;

use super::{Claims, SignUpClaims};

use jsonwebtoken::{DecodingKey, Validation};

#[instrument(level = "debug", skip(req))]
pub(super) async fn sign_up(mut req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    // 注册成功返回1,否则返回0
    let res = HttpResponse::Ok();

    // 获取参数
    let body = req
        .body_mut()
        .data()
        .await
        .ok_or_else(|| tracing::debug!("Body is None!"))?
        .map_err(|e| tracing::debug!("{}", e))?;

    let param: SignUpParam<&str> = serde_json::from_slice(&body)
        .map_err(|e| tracing::debug!("Error when parsing body: {}", e))?;
    tracing::info!("{:#?}", param);

    // 获取jwt
    let jwt = req
        .headers()
        .get(AUTHORIZATION)
        .map(AsRef::as_ref)
        .map(std::str::from_utf8)
        .ok_or_else(|| tracing::debug!("Authorization is none"))?
        .map_err(|e| tracing::debug!("{}", e))?;

    // 验证jwt
    let mut validation = Validation::default();
    validation.validate_exp = true;
    let claims = jsonwebtoken::decode::<Claims<SignUpClaims<String>>>(
        &jwt,
        &DecodingKey::from_secret(SIGN_UP_KEY),
        &validation,
    )
    .map(|token| token.claims)
    .map_err(|e| tracing::debug!("Error when decode jwt: {}", e))?;

    // 验证邮箱（验证码不需要再判断是否相等，因为验证码已经参与了密钥的构建）
    if claims.payload.verify_code != param.verify_code || claims.payload.email != param.email {
        tracing::debug!("verify_code or email not equal");
        return Err(crate::ResErr::Any);
    }

    // 注册
    let pool = MyPool::new();
    let user = BaseUser {
        email: param.email,
        username: param.username,
        password: param.password,
    };

    Ok(res.json(
        sql_sign_up(pool, user)
            .await
            .map_err(|e| tracing::debug!("{}", e))?,
    ))
}

pub(super) const SIGN_UP_KEY: &[u8] = b"MYSTU_SIGN_UP_KEY";

async fn sql_sign_up<'a, T>(
    pool: &Pool<Postgres>,
    user: BaseUser<T>,
) -> std::result::Result<i32, sqlx::Error>
where
    T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Sync + Send + 'a,
{
    sqlx::query_scalar(
        "WITH ins AS ( \
                INSERT INTO users (email, username, password) VALUES ($1, $2, $3) \
                ON CONFLICT (username) DO NOTHING \
                RETURNING 1 AS res\
            ) \
            SELECT COALESCE((SELECT res FROM ins), -1)",
    )
    .bind(user.email)
    .bind(user.username)
    .bind(user.password)
    .fetch_one(pool)
    .await
}

#[derive(Debug, sqlx::FromRow)]
pub(super) struct BaseUser<T> {
    pub email: T,
    pub username: T,
    pub password: T,
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub(super) struct SignUpParam<T> {
    pub email: T,
    pub username: T,
    pub password: T,
    pub verify_code: T,
}
