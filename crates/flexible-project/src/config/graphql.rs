use actix_web::{get, route, web, Responder};
use actix_web_lab::respond::Html;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::model::Schema;

/// Provides configuration for GraphQL endpoints of the Flexible Project system.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playground).service(graphql);
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, request: GraphQLRequest) -> GraphQLResponse {
    let request = request.into_inner();
    let response = schema.execute(request).await;
    response.into()
}

/// GraphiQL playground UI
#[get("/graphiql")]
async fn playground() -> impl Responder {
    let config = GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql");
    Html(playground_source(config))
}
