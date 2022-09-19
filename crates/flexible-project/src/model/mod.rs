//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject};

mod id;
mod user;

/// GraphQL schema of the Flexible Project system.
pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

/// GraphQL root query object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery);
