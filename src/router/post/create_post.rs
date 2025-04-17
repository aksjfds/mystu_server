use std::borrow::Cow;

use glacier::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{prelude::sql::*, router::user};

#[instrument(level = "debug", skip(req))]
pub async fn create_post(mut req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    let payload: user::LoginPayload = req
        .headers()
        .get(AUTHORIZATION)
        .map(|token| user::verify_token(token, user::ACCESS_KEY))
        .ok_or_else(|| tracing::debug!("Authorization is none"))??;

    req.body_mut()
        .data()
        .await
        .ok_or_else(|| tracing::debug!("Body is None!"))?
        .as_ref()
        .map(|data| serde_json::from_slice::<BasePost<Cow<'_, str>>>(&data))
        .map_err(|e| tracing::debug!("{}", e))?
        .map_err(|e| tracing::debug!("Error when parsing body: {}", e))
        .and_then(|post: BasePost<Cow<'_, str>>| {
            if payload.username == post.author {
                Ok(post)
            } else {
                Err(())
            }
        })
        .inspect(|post| tracing::info!("this post try to upload: {:#?}", post))
        .async_map(sql_create_post)
        .await
        .map(|post_id| HttpResponse::Ok().json(post_id))
        .map_err(|_| crate::ResErr::Any)
}

async fn sql_create_post<'a, T>(post: BasePost<T>) -> i32
where
    T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Sync + Send + 'a,
{
    let pool = MyPool::new();

    sqlx::query_scalar(
        "WITH ins AS ( \
            INSERT INTO posts(title, author, content) VALUES ($1, $2, $3) \
            ON CONFLICT (title) DO NOTHING \
            RETURNING id \
        )\
        SELECT COALESCE((SELECT id FROM ins), -1)",
    )
    .bind(post.title)
    .bind(post.author)
    .bind(post.content)
    .fetch_one(pool)
    .await
    .map_err(|e| tracing::debug!("{:#?}", e))
    .unwrap_or(0)
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BasePost<T> {
    pub title: T,
    pub author: T,
    pub content: T,
}

#[test]
fn test() {
    let post = BasePost {
        title: "你好，世界\nHello, World",
        author: "aksjfds",
        content: "你好，世界\nHello, World",
    };

    let str = serde_json::to_string(&post).unwrap();
    println!("{:#?}", str);

    let post: BasePost<Cow<'_, str>> = serde_json::from_str(&str).unwrap();
    println!("{:#?}", post);
}
