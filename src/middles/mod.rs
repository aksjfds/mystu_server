use glacier::prelude::*;
use redis::AsyncCommands;

use crate::{ResErr, TOO_MANY_REQUEST, sql::MyPool};

pub fn get(req: HttpRequest) -> Result<HttpRequest, ()> {
    if req.method() == "GET" {
        Ok(req)
    } else {
        Err(())
    }
}

pub fn post(req: HttpRequest) -> Result<HttpRequest, ()> {
    if req.method() == "POST" {
        Ok(req)
    } else {
        Err(())
    }
}

pub struct IpLogger;
impl IpLogger {
    // 一个过滤器，用来限制重要函数的访问速率
    // 每次访问后在redis设置一个过期时间，
    // 当再次访问时，若未过期则放弃该请求
    pub async fn limit(
        req: HttpRequest,
        func_name: &str,
        seconds: u64,
    ) -> Result<HttpRequest, crate::ResErr> {
        let key = format!("{}: {}", req.ip, func_name);

        let mut con = MyPool::redis_conn().await.unwrap();
        let is_expire: Option<u8> = con.get(&key).await.unwrap();
        match is_expire {
            Some(_) => {
                tracing::info!("this req is being limited: {:#?}", key);

                req.global_val
                    .error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Err(ResErr::Detail(TOO_MANY_REQUEST))
            }
            None => {
                let _: () = con.set_ex(key, 0u8, seconds).await.unwrap();
                Ok(req)
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn Ban(ip: &str) {
        // sudo ufw status
        // sudo ufw deny from ip [to any port port]
        // sudo ufw status numbered
        // sudo ufw delete 2
        use std::process::Command;
        let command = Command::new("sudo")
            .arg("ufw")
            .arg("deny")
            .arg("from")
            .arg(ip)
            .output()
            .unwrap();

        println!("{:#?}", String::from_utf8(command.stdout));
        println!("{:#?}", String::from_utf8(command.stderr));
    }
}
