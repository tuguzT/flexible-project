#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

//! Flexible Project server.

use std::net::SocketAddr;

use anyhow::{Context, Result};
use async_graphql::extensions;
use axum::{extract::Extension, Router, Server};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use self::route::graphql_routes;
use self::schema::build_schema;

pub mod model;
pub mod route;
pub mod schema;

/// Entry point of the server.
#[tokio::main]
pub async fn main() -> Result<()> {
    if cfg!(debug_assertions) {
        dotenv::dotenv().with_context(|| ".env file not found")?;
    }

    let log_directives = std::env::var("RUST_LOG").with_context(|| "RUST_LOG must be set")?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_directives))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .with_context(|| "failed to init tracing")?;

    let schema = build_schema()
        .await?
        .extension(extensions::Tracing)
        .finish();
    tracing::debug!("GraphQL schema SDL:\n{}", schema.sdl());

    let app = Router::new()
        .merge(graphql_routes())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(schema));

    let addr = &SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Catches a signal to shut down the server.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}
