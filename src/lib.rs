pub mod middles;

mod error;
pub use error::*;

pub mod prelude;
pub mod router;
pub mod sql;
pub mod tool;
pub mod log;

pub static MY_IP: std::sync::LazyLock<std::net::SocketAddr> =
    std::sync::LazyLock::new(|| tool::my_ip());
