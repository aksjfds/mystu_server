use criterion::{Criterion, criterion_group, criterion_main};
use mystu_server::sql::MyPool;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
struct Post {
    title: String,
    author: String,
    content: String,
}

async fn bench_direct_json() -> serde_json::Value {
    let pool = MyPool::new();

    sqlx::query_scalar(
        "SELECT to_jsonb(posts) FROM posts LIMIT 1",
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn bench_vec_post() -> serde_json::Value {
    let pool = MyPool::new();

    let posts: Vec<Post> = sqlx::query_as("SELECT * FROM posts LIMIT 1")
        .fetch_all(pool)
        .await
        .unwrap();
    serde_json::to_value(posts).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("direct_json", |b| {
        b.iter(|| rt.block_on(bench_direct_json()))
    });
    c.bench_function("vec_post", |b| b.iter(|| rt.block_on(bench_vec_post())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
