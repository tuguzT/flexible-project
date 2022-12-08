//! GraphQL routes of the Flexible Project system server.

use std::str;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

use crate::model::user::UserToken;
use crate::schema::Schema;

/// All GraphQL routes of the Flexible Project system.
pub fn all_routes() -> Router {
    Router::new().merge(graphql()).merge(graphiql_playground())
}

/// The main GraphQL endpoint of the Flexible Project system.
pub fn graphql() -> Router {
    async fn handler(
        headers: HeaderMap,
        schema: Extension<Schema>,
        request: GraphQLRequest,
    ) -> GraphQLResponse {
        let mut request = request.into_inner();
        if let Some(token) = authorization_token(&headers) {
            request = request.data(token);
        }
        schema.execute(request).await.into()
    }

    Router::new().route("/graphql", post(handler))
}

/// GraphiQL playground UI of the Flexible Project system.
pub fn graphiql_playground() -> Router {
    async fn handler() -> impl IntoResponse {
        let config = GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql");
        Html(playground_source(config))
    }

    Router::new().route("/graphiql", get(handler))
}

/// Retrieve Bearer token from the "Authorization" header.
fn authorization_token(headers: &HeaderMap) -> Option<UserToken> {
    let header = headers.get("Authorization")?.as_bytes();
    if header.get(..7)? != b"Bearer " {
        return None;
    }
    let token = str::from_utf8(header.get(7..)?).ok()?.to_string();
    let token = UserToken { token };
    Some(token)
}
