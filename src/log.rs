/* --------------------------------- // 日志处理 -------------------------------- */
pub struct Logging;
impl Logging {
    pub fn start(max_level: &str, file_path: Option<&str>) {
        use std::fs::{File, OpenOptions};
        use std::str::FromStr;
        use tracing::level_filters::LevelFilter;
        use tracing_subscriber::EnvFilter;

        let level = tracing::Level::from_str(max_level).unwrap();

        let file = file_path.map(|file_path| {
            let options: _ = OpenOptions::new().append(true).open(&file_path);
            match options {
                Ok(f) => f,
                Err(_) => File::create(&file_path).unwrap(),
            }
        });

        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::DEBUG.into())
            .from_env()
            .unwrap()
            .add_directive("h2=info".parse().unwrap())
            .add_directive("rustls=info".parse().unwrap());

        let subscriber = tracing_subscriber::fmt()
            .with_max_level(level)
            .with_env_filter(filter)
            .with_target(false);

        match file {
            Some(file) => subscriber.with_writer(file).with_ansi(false).init(),
            None => subscriber.init(),
        }
    }
}
