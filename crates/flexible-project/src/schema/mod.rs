//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use async_graphql::{EmptySubscription, MergedObject};

use crate::data::create_user_repository;

pub mod user;

/// GraphQL schema of the Flexible Project system.
pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// Builder of the Flexible Project system [schema](Schema).
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription>;

/// Builds the [schema](Schema) for the Flexible Project system.
///
/// Returns a [builder](SchemaBuilder) to allow users to customize it.
pub fn build_schema() -> SchemaBuilder {
    let user_repository = create_user_repository();
    Schema::build(
        Query::default(),
        Mutation::default(),
        EmptySubscription::default(),
    )
    .data(user_repository)
}

/// GraphQL root query object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery);

/// GraphQL root mutation object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutation);
