#![allow(unused)]

use crate::sql::{MyPool, REDIS_CLIENT};
use redis::{AsyncCommands, Commands};
use sqlx::Row;

async fn test_func() {
    let pool = MyPool::new();
}

#[tokio::test]
async fn test() {
    let mut conn = MyPool::redis_conn().await.unwrap();

    // let _: () = conn.set("key", "value").unwrap();

    let res: String = conn.get("key").await.unwrap();
    println!("{:#?}", res);
}
