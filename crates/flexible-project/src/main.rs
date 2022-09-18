#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server.

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use flexible_project::config::user_config;
use flexible_project::user_repository;

/// Entry point of the server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env file parsing failure");
    pretty_env_logger::init();

    let user_repository = user_repository();
    let user_config = user_config(user_repository);

    HttpServer::new(move || {
        let logger = Logger::default();
        let user_config = user_config.clone();
        App::new().wrap(logger).configure(user_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
