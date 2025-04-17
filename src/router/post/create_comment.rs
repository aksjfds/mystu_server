use glacier::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{prelude::sql::*, router::user};

#[instrument(level = "debug", skip(req))]
pub async fn create_comment(mut req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
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
        .map(|data| serde_json::from_slice::<BaseComment<&str>>(&data))
        .map_err(|e| tracing::debug!("{}", e))?
        .map_err(|e| tracing::debug!("Error when parsing body: {}", e))
        .and_then(|post: BaseComment<&str>| {
            if payload.username == post.username {
                Ok(post)
            } else {
                Err(())
            }
        })
        .inspect(|post| tracing::info!("this post try to upload: {:#?}", post))
        .async_map(sql_create_comment)
        .await
        .map(|post_id| HttpResponse::Ok().json(post_id))
        .map_err(|_| crate::ResErr::Any)
}

async fn sql_create_comment<'a, T>(comment: BaseComment<T>) -> i32
where
    T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Sync + Send + 'a,
{
    match comment.reply_to {
        Some(reply_to) => sqlx::query_scalar(
            "INSERT INTO comments ( \
    post_id, parent_id, reply_to, username, content ) \
    VALUES ( $1, $2, $3, $4, $5 ) \
    RETURNING comment_id",
        )
        .bind(comment.post_id)
        .bind(comment.parent_id)
        .bind(reply_to)
        .bind(comment.username)
        .bind(comment.content)
        .fetch_one(MyPool::new())
        .await
        .map_err(|e| tracing::debug!("{:#?}", e))
        .unwrap_or(0),
        None => sqlx::query_scalar(
            "INSERT INTO comments ( \
        post_id, parent_id, username, content ) \
        VALUES ( $1, $2, $3, $4 ) \
        RETURNING comment_id",
        )
        .bind(comment.post_id)
        .bind(comment.parent_id)
        .bind(comment.username)
        .bind(comment.content)
        .fetch_one(MyPool::new())
        .await
        .map_err(|e| tracing::debug!("{:#?}", e))
        .unwrap_or(0),
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BaseComment<T> {
    pub post_id: i32,
    // comment_id:i32,      // 由数据库生成
    pub parent_id: i32,
    pub reply_to: Option<T>,
    pub username: T,
    pub content: T,
}
