//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use async_graphql::{EmptySubscription, MergedObject};

use crate::data::create_user_repository;
use crate::model::Node;

pub mod node;
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

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    Schema::build(query, mutation, subscription)
        .register_output_type::<Node>()
        .data(user_repository)
}

/// Root query object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery, node::NodeQuery);

/// Root mutation object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutation);

/// Root subscription object of the Flexible Project system.
pub type Subscription = EmptySubscription;
