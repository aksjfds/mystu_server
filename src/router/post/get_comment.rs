use glacier::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::sql::MyPool;

#[instrument(level = "debug", skip(req))]
pub async fn get_comment(req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    req.param::<GetCommentParam>()
        .async_map(sql_get_comment)
        .await
        .map_err(|_| crate::ResErr::Any)
        .map(|comments| HttpResponse::Ok().json(comments))
}

async fn sql_get_comment(
    GetCommentParam {
        post_id,
        parent_id,
        last_id,
    }: GetCommentParam,
) -> Vec<Comment>
where
{
    let sql = "\
        SELECT comment_id, reply_to, username, content, \
        to_char(time, 'YYYY-MM-DD HH24:MI') AS time FROM comments \
        WHERE post_id = $1 AND parent_id = $2 AND comment_id > $3 LIMIT $4";

    sqlx::query_as(sql)
        .bind(post_id)
        .bind(parent_id)
        .bind(last_id)
        .bind(5)
        .fetch_all(MyPool::new())
        .await
        .unwrap()
}

#[tokio::test]
async fn test() {
    let param = GetCommentParam {
        post_id: 1,
        parent_id: -1,
        last_id: 1,
    };

    let comments = sql_get_comment(param).await;
    println!("{:#?}", comments);
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct Comment {
    comment_id: i32,
    reply_to: Option<String>,
    username: String,
    content: String,
    time: String,
}

#[derive(Deserialize)]
struct GetCommentParam {
    post_id: i32,
    parent_id: i32, // 用 -1 代替 None
    last_id: i32,
    // limit: i32,         // 不要 limit,强制5个
}
