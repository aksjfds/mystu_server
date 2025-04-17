use glacier::prelude::*;
use mystu_server::middles::IpLogger;
use mystu_server::prelude::*;

const VISIT_LIMIT: u64 = 1; // 两次请求间的最小时间间隔 (秒)
const ERROR_LIMIT: u8 = 10; // 可允许的异常请求次数

const CORS: &str = "https://aksjfds.github.io";

async fn router(mut req: HttpRequest) -> HttpResponse {
    if req.global_val.log(VISIT_LIMIT, ERROR_LIMIT) != 0 {
        IpLogger::Ban(req.ip.as_str());

        return HttpResponse::Ok().json(400);
    }

    if req.method() == "OPTIONS" {
        return HttpResponse::Ok()
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS)
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization");
    }

    let res = match req.next_route().as_deref() {
        Some("/user") => user::route(req).await,
        Some("/post") => post::route(req).await,
        _ => Ok(HttpResponse::Ok().plain("404")),
    };

    match res {
        Ok(res) => res
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS)
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization"),
        Err(res_err) => {
            let res = HttpResponse::Ok()
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, CORS)
                .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization");

            match res_err {
                mystu_server::ResErr::Any => res.easy_json([b'0']),
                mystu_server::ResErr::Detail(code) => res.json(code),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    mystu_server::log::Logging::start("debug", None);

    let glacier = GlacierBuilder::bind(("0.0.0.0", 443))
        .server(router)
        .build()
        .await;
    glacier.run().await.unwrap();

    std::thread::park();
}
