//! Data model of the gateway service.

use async_graphql::{EmptyMutation, MergedObject, MergedSubscription};

use self::{
    notification::{NotificationQuery, NotificationSubscription},
    user::UserQuery,
    workspace::WorkspaceQuery,
};

pub mod notification;
pub mod user;
pub mod workspace;

/// GraphQL schema of the service.
pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

/// Builder of the GraphQL [schema](Schema) of the service.
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, Subscription>;

/// Root query object of the service.
#[derive(Debug, MergedObject, Default)]
pub struct Query(UserQuery, WorkspaceQuery, NotificationQuery);

/// Root mutation object of the service.
pub type Mutation = EmptyMutation;

/// Root subscription object of the service.
#[derive(Debug, MergedSubscription, Default)]
pub struct Subscription(NotificationSubscription);
