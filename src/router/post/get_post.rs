use glacier::prelude::*;
use tracing::instrument;

use crate::prelude::param::*;
use crate::prelude::sql::*;

#[instrument(level = "debug", skip(req))]
pub async fn get_post(req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    req.param::<GetPostParam>()
        .async_map(sql_get_post)
        .await
        .map_err(|_| crate::ResErr::Any)
        .map(|posts| HttpResponse::Ok().json(posts))
}

pub async fn sql_get_post(param: GetPostParam) -> Vec<Post> {
    sqlx::query_as(
        "SELECT id, title, author, \
        to_char(time, 'YYYY-MM-DD HH24:MI') AS time, \
        content FROM posts WHERE id > $1 ORDER BY id ASC LIMIT $2",
    )
    .bind(param.last_id)
    .bind(param.limit)
    .fetch_all(MyPool::new())
    .await
    .unwrap()
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    // pub time: chrono::NaiveDateTime,
    pub time: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetPostParam {
    pub last_id: i32,
    pub limit: i16,
}
