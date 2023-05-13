//! Data model of the gateway service.

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject};

use self::{user::UserQuery, workspace::WorkspaceQuery};

pub mod user;
pub mod workspace;

/// GraphQL schema of the service.
pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

/// Builder of the GraphQL [schema](Schema) of the service.
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, Subscription>;

/// Root query object of the service.
#[derive(Debug, MergedObject, Default)]
pub struct Query(UserQuery, WorkspaceQuery);

/// Root mutation object of the service.
pub type Mutation = EmptyMutation;

/// Root subscription object of the service.
pub type Subscription = EmptySubscription;
