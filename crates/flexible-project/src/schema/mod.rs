//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use anyhow::{Context, Result};
use async_graphql::{EmptySubscription, MergedObject};

use crate::model::node::Node;

use self::di::{interactor::token::Secret, SchemaModule};

pub mod di;
pub mod node;
pub mod user;

mod ext;

/// GraphQL schema of the Flexible Project system.
pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

/// Builder of the Flexible Project system [schema](Schema).
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, Subscription>;

/// Builds the [schema](Schema) for the Flexible Project system.
///
/// Returns a [builder](SchemaBuilder) to allow users to customize it.
pub async fn build_schema() -> Result<SchemaBuilder> {
    let token_secret = std::env::var("JWT_SECRET")
        .with_context(|| "JWT_SECRET environment variable must be set")?;
    let module = SchemaModule::builder()
        .with_component_parameters::<Secret>(token_secret)
        .build();

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    let schema_builder = Schema::build(query, mutation, subscription)
        .data(module)
        .register_output_type::<Node>();
    Ok(schema_builder)
}

/// Root query object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery, node::NodeQuery);

/// Root mutation object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutation);

/// Root subscription object of the Flexible Project system.
pub type Subscription = EmptySubscription;
