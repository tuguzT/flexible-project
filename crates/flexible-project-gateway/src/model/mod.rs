//! Data model of the gateway service.

use async_graphql::{
    MergedObject, MergedSubscription, Schema as GraphQLSchema,
    SchemaBuilder as GraphQLSchemaBuilder,
};

use self::{
    methodology::{MethodologyMutation, MethodologyQuery},
    notification::{NotificationMutation, NotificationQuery, NotificationSubscription},
    user::{UserMutation, UserQuery},
    workspace::{WorkspaceMutation, WorkspaceQuery},
};

pub mod methodology;
pub mod notification;
pub mod user;
pub mod workspace;

/// GraphQL schema of the service.
pub type Schema = GraphQLSchema<Query, Mutation, Subscription>;

/// Builder of the GraphQL [schema](Schema) of the service.
pub type SchemaBuilder = GraphQLSchemaBuilder<Query, Mutation, Subscription>;

/// Root query object of the service.
#[derive(Debug, MergedObject, Default)]
pub struct Query(
    UserQuery,
    WorkspaceQuery,
    MethodologyQuery,
    NotificationQuery,
);

/// Root mutation object of the service.
#[derive(Debug, MergedObject, Default)]
pub struct Mutation(
    UserMutation,
    WorkspaceMutation,
    MethodologyMutation,
    NotificationMutation,
);

/// Root subscription object of the service.
#[derive(Debug, MergedSubscription, Default)]
pub struct Subscription(NotificationSubscription);
