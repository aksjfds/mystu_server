use redis::{AsyncCommands, aio::MultiplexedConnection};
use sqlx::{Pool, Postgres};

pub static mut POOL: std::sync::LazyLock<sqlx::Pool<sqlx::Postgres>> =
    std::sync::LazyLock::new(|| {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy("postgres://postgres:1@localhost:5432/mystu")
            .unwrap()
    });

pub static REDIS_CLIENT: std::sync::LazyLock<redis::Client> =
    std::sync::LazyLock::new(|| redis::Client::open("redis://127.0.0.1/").unwrap());

pub struct MyPool;
impl MyPool {
    pub fn new() -> &'static Pool<Postgres> {
        unsafe { &*POOL }
    }

    pub async fn redis_conn() -> Result<MultiplexedConnection, redis::RedisError> {
        REDIS_CLIENT.get_multiplexed_async_connection().await
    }

    pub async fn redis_set_ex<'a, K, V>(
        key: K,
        value: V,
        duration: chrono::Duration,
    ) -> Result<(), redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
        V: redis::ToRedisArgs + Send + Sync + 'a,
    {
        use redis::AsyncCommands;

        REDIS_CLIENT
            .get_multiplexed_async_connection()
            .await?
            .set_ex(key, value, duration.num_seconds() as u64)
            .await
    }

    pub async fn redis_get_del<'a, K, RV>(key: K) -> Result<RV, redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
        RV: redis::FromRedisValue,
    {
        REDIS_CLIENT
            .get_multiplexed_async_connection()
            .await?
            .get_del(key)
            .await
    }
}

#[tokio::test]
async fn test() {
    let mut pipe = redis::pipe();
    pipe.hset("key", "field", "value");
    pipe.hexpire("key", 3, redis::ExpireOption::NONE, "field");

    let _: () = pipe
        .query_async(&mut MyPool::redis_conn().await.unwrap())
        .await
        .unwrap();
}
