//! Flexible Project API Gateway backend microservice binary.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use async_graphql::extensions::Tracing;
use axum::{Extension, Router, Server};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use self::{
    routes::{graphql, health},
    schema::{Mutation, Query, Schema, Subscription},
};

pub mod routes;
pub mod schema;

/// Entry point of the server.
#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .with_context(|| "failed to init tracing subscriber")?;

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    let schema = Schema::build(query, mutation, subscription)
        .extension(Tracing)
        .finish();
    tracing::debug!("GraphQL schema SDL:\n{}", schema.sdl());

    let app = Router::new()
        .merge(graphql::graphiql())
        .merge(graphql::graphql())
        .layer(Extension(schema))
        .merge(health::health())
        .layer(TraceLayer::new_for_http());

    let addr = &SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .with_context(|| "failed to init server")?;
    Ok(())
}

/// Catches a signal to shut down the server.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}
