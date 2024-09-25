use dotenvy::dotenv;
use ntex::web;
use ntex_cors::Cors;
use std::env;
use tracing::info;
use tracing_subscriber;

mod handler;
mod utils;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // use Info level for release
    // use Debug level for debug
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    // 设置RUST_LOG环境变量
    env::set_var("RUST_LOG", log_level);

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // load environment variables from os
    dotenv().ok();

    let app_port = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("must be a number");

    info!("Starting server on port {}", app_port);

    // web::HttpServer can be shutdown gracefully.
    web::HttpServer::new(move || {
        web::App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .finish(),
            )
            // enable logger
            .wrap(web::middleware::Logger::default())
            // enable default headers
            .wrap(web::middleware::DefaultHeaders::new().header("content-type", "application/json"))
            // enable Compression, A response's Content-Encoding header defaults to ContentEncoding::Auto, which performs automatic content compression negotiation based on the request's Accept-Encoding header.
            // should add "compress" feature to the Cargo.toml
            .wrap(web::middleware::Compress::default())
            .configure(handler::init)
    })
    .bind(("0.0.0.0", app_port))?
    .run()
    .await
}
