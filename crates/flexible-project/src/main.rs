#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server.

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription};
use flexible_project::config::graphql_config;
use flexible_project::data::user_repository;
use flexible_project::model::{Query, Schema};

/// Entry point of the server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env file should be present and valid");
    pretty_env_logger::init();

    let user_repository = user_repository();
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(user_repository)
        .finish();
    let schema = web::Data::new(schema);

    log::debug!("{}", schema.sdl());

    HttpServer::new(move || {
        let schema = schema.clone();
        let logger = Logger::default();
        let cors = Cors::permissive();

        App::new()
            .app_data(schema)
            .configure(graphql_config)
            .wrap(cors)
            .wrap(logger)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
