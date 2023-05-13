//! Definitions of health routes.

use axum::{response::IntoResponse, routing::get, Router};

/// Health route with path of `/health`.
pub fn health() -> Router {
    async fn handler() -> impl IntoResponse {
        "Healthy"
    }

    Router::new().route("/health", get(handler))
}
