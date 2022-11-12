#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

//! Flexible Project server.

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use async_graphql::extensions;

use self::config::graphql_config;
use self::schema::build_schema;

mod config;
mod model;
mod schema;

/// Entry point of the server.
#[actix_web::main]
async fn main() -> Result<()> {
    if dotenv::dotenv().is_err() {
        log::info!(".env file not found, server may panic unexpectedly");
    }
    log_panics::init();
    pretty_env_logger::try_init()?;

    let schema = build_schema().await?.extension(extensions::Logger).finish();
    let schema = web::Data::new(schema);
    log::debug!("GraphQL schema SDL:\n{}", schema.sdl());

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
    .await?;
    Ok(())
}
