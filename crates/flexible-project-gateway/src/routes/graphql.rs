//! Definitions of GraphQL routes.

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

use crate::model::Schema;

/// The main GraphQL endpoint which handles all input requests
/// with path of `/graphql`.
pub fn graphql() -> Router {
    async fn handler(schema: Extension<Schema>, request: GraphQLRequest) -> GraphQLResponse {
        let request = request.into_inner();
        schema.execute(request).await.into()
    }

    Router::new().route("/graphql", post(handler))
}

/// Presents GraphiQL IDE to user with path of `/graphiql`.
pub fn graphiql() -> Router {
    async fn handler() -> impl IntoResponse {
        let source = GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/graphql")
            .finish();
        Html(source)
    }

    Router::new().route("/graphiql", get(handler))
}
