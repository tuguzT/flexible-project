//! Definition of the service schema and its components.

use async_graphql::{EmptyMutation, EmptySubscription, Object};

/// GraphQL schema of the service.
pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

/// Builder of the GraphQL [schema](Schema) of the service.
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, Subscription>;

/// Root query object of the service.
#[derive(Debug, Default)]
pub struct Query;

#[Object]
impl Query {
    /// Answer to the Ultimate Question of Life, the Universe, and Everything
    async fn answer(&self) -> i32 {
        42
    }
}

/// Root mutation object of the service.
pub type Mutation = EmptyMutation;

/// Root subscription object of the service.
pub type Subscription = EmptySubscription;
