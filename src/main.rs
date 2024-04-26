#[macro_use]
extern crate actix_web;

use chimes_auth::ChimesAuthorization;
use chimes_utils::{get_rbatis, AppConfig, WebServerConfig};
// #[macro_use]
// extern crate wechat;

// use awc::Client;

use actix_cors::Cors;
use actix_web::http::{header, StatusCode};
use actix_web::{middleware, App, HttpRequest, HttpResponse, HttpServer, Result};

mod entity;
mod handler;
mod query;

mod utils;

use crate::utils::{AppEntryCollect, ChimesUserAuthService};

#[get("/")]
async fn index_handler(_req: HttpRequest) -> Result<HttpResponse> {
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("App is running."))
}

/// 应用启动入口
//#[actix_web::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> std::io::Result<()> {
    // 加载配置文件
    let conf_path = std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned()
        + "/conf/app.yml";
    log::info!("Current Path: {}", conf_path);
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // env_logger::init();

    match fast_log::init(fast_log::config::Config::new().console()) {
        Ok(_) => {}
        Err(err) => {
            log::warn!("An error occurred on the Logger initializing. {}", err);
        }
    };

    // // 加载配置信息
    let mut appconf = AppConfig::get().lock().unwrap();
    appconf.load_yaml(&conf_path.clone());
    let conf = appconf.clone();
    drop(appconf);

    let _rb = get_rbatis();

    // 启动web服务
    start_web_server(&conf.webserver_conf).await
}

/// web服务启动
async fn start_web_server(webconf: &WebServerConfig) -> std::io::Result<()> {
    // 设置服务器运行ip和端口信息
    let temp_path = webconf.upload_temp_path.clone();
    let ip = format!("{}:{}", "0.0.0.0", webconf.port.clone());
    log::info!("App is listening on {}.", ip.clone());
    // 启动一个web服务
    let cuas = ChimesUserAuthService { system_user: None };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                ChimesAuthorization::new(cuas.clone())
                    .header_key(&"Authorization".to_string())
                    .allow(&"/api/v1/auth/login".to_string())
                    .allow(&"/api/v1/auth/code".to_string())
                    .allow(&"/api/v1/auth/logout".to_string())
                    .allow(&"/api/v1/healthcheck".to_string()),
            )
            .wrap(
                // 设置允许跨域请求
                Cors::default()
                    .allowed_origin("http://localhost:8013")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(
                awmp::PartsConfig::default()
                    .with_file_limit(1000000000)
                    .with_temp_dir(temp_path.as_str()),
            )
            .service(index_handler)
            .register_handlers()
    })
    .bind(ip)?
    .run()
    .await
}
