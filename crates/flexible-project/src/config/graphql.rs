use std::str;

use actix_web::{get, http::header::HeaderMap, route, web, HttpRequest, Responder};
use actix_web_lab::respond::Html;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::model::user::UserToken;
use crate::schema::Schema;

/// Provides configuration for GraphQL endpoints of the Flexible Project system.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playground).service(graphql);
}

/// GraphiQL playground UI of the Flexible Project system.
#[get("/graphiql")]
async fn playground() -> impl Responder {
    let config = GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql");
    Html(playground_source(config))
}

/// The main GraphQL endpoint of the Flexible Project system.
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    schema: web::Data<Schema>,
    request: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_request = gql_request.into_inner();
    if let Some(token) = authorization_token(request.headers()) {
        gql_request = gql_request.data(token);
    }
    schema.execute(gql_request).await.into()
}

fn authorization_token(headers: &HeaderMap) -> Option<UserToken> {
    let header = headers.get("Authorization")?.as_bytes();
    if header.get(..7)? != b"Bearer " {
        return None;
    }
    let token = str::from_utf8(header.get(7..)?).ok()?.to_string();
    let token = UserToken { token };
    Some(token)
}
