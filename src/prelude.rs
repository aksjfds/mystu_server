pub use crate::MY_IP;
pub use crate::router::*;

pub mod sql {
    pub use crate::sql::MyPool;
    pub use sqlx::postgres::PgQueryResult;
    pub use sqlx::{Pool, Postgres};
}

pub mod param {
    pub use serde::{Deserialize, Serialize};
}
