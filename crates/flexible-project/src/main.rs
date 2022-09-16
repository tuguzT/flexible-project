#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server.

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use flexible_project::routes::{all_users, save_user, user_by_id};
use flexible_project::user_repository;

/// Entry point of the server.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env file parsing failure");
    pretty_env_logger::init();

    let user_repository = web::Data::new(user_repository());

    HttpServer::new(move || {
        let user_repository = user_repository.clone();
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(user_repository)
            .service(all_users)
            .service(user_by_id)
            .service(save_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
