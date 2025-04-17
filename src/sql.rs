use sqlx::{Pool, Postgres};

const PGSQL_URL: &str = "postgresql://postgres.yihtetapipznguhpldpj:qi7lAEMRggQkLvt2@aws-0-ap-southeast-1.pooler.supabase.com:5432/postgres";

const REDIS_URL: &str = "rediss://default:AUnAAAIjcDE4ZGQyMjZhMDg4M2I0NzQyYTU3ZWFjODk4YzI1OGY4OHAxMA@next-fish-18880.upstash.io:6379";

pub static mut POOL: std::sync::LazyLock<sqlx::Pool<sqlx::Postgres>> =
    std::sync::LazyLock::new(|| {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            // .connect_lazy("postgres://postgres:1@localhost:5432/mystu")
            .connect_lazy(PGSQL_URL)
            .unwrap()
    });

pub static REDIS_CLIENT: std::sync::LazyLock<redis::Client> =
    std::sync::LazyLock::new(|| redis::Client::open(REDIS_URL).unwrap());

pub struct MyPool;
impl MyPool {
    pub fn new() -> &'static Pool<Postgres> {
        unsafe { &*POOL }
    }

    pub fn redis_conn() -> Result<redis::Connection, redis::RedisError> {
        REDIS_CLIENT.get_connection()
    }

    pub fn redis_set_ex<'a, K, V>(
        key: K,
        value: V,
        duration: chrono::Duration,
    ) -> Result<(), redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
        V: redis::ToRedisArgs + Send + Sync + 'a,
    {
        use redis::Commands;

        REDIS_CLIENT
            .get_connection()?
            .set_ex(key, value, duration.num_seconds() as u64)
    }

    pub fn redis_get_del<'a, K, RV>(key: K) -> Result<RV, redis::RedisError>
    where
        K: redis::ToRedisArgs + Send + Sync + 'a,
        RV: redis::FromRedisValue,
    {
        use redis::Commands;

        REDIS_CLIENT.get_connection()?.get_del(key)
    }
}

#[test]
fn test() {
    use redis::Commands;

    let mut con = REDIS_CLIENT.get_connection().unwrap();
    let _: () = con.set("key", "value").unwrap();

    let _a: String = con.get("key").unwrap();
    println!("{:#?}", _a);
}
