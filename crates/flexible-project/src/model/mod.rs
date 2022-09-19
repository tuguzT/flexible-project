//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject};

use crate::data::user_repository;

mod id;
mod user;

/// GraphQL schema of the Flexible Project system.
pub type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

/// Creates the [schema](Schema) instance for the Flexible Project system.
pub fn create_schema() -> Schema {
    let user_repository = user_repository();
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(user_repository)
        .finish()
}

/// GraphQL root query object of the Flexible Project system.
#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery);
