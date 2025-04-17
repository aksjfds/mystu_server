use crate::middles::*;
use create_comment::create_comment;
use glacier::prelude::*;

mod create_comment;
mod create_post;
mod get_comment;
mod get_post;

use create_post::create_post;
use get_comment::get_comment;
use get_post::get_post;

pub async fn route(mut req: HttpRequest) -> Result<HttpResponse, crate::ResErr> {
    match req.next_route().as_deref() {
        Some("/get_post") => req.filter_with(get)?.async_apply(get_post).await,
        Some("/create_post") => {
            req.filter_with(post)?
                .async_filter(|req| IpLogger::limit(req, "createPost", 60))
                .await?
                .async_apply(create_post)
                .await
        }
        Some("/get_comment") => req.filter_with(get)?.async_apply(get_comment).await,
        Some("/create_comment") => {
            req.filter_with(post)?
                .async_filter(|req| IpLogger::limit(req, "createComment", 60))
                .await?
                .async_apply(create_comment)
                .await
        }
        _ => Ok(HttpResponse::Ok().plain("404")),
    }
}
