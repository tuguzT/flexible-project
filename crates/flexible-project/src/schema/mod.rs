//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use std::sync::Arc;

use async_graphql::{EmptySubscription, MergedObject};
use fp_data::data_source::local::{Client, LocalUserDataSource};
use fp_data::interactor::{
    CreateUser, DeleteUser, FilterUsers, FindNode, GUIDGenerator, UpdateUser,
    UserCredentialsVerifier,
};
use fp_data::repository::user::UserRepository;
use fp_data::repository::Error as RepositoryError;
use fp_data::Error;

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
pub async fn build_schema() -> Result<SchemaBuilder, Error> {
    let client = Client::new().map_err(RepositoryError::from)?;
    let database = client.0.database("flexible-project");
    let user_data_source = LocalUserDataSource::new(database)
        .await
        .map_err(RepositoryError::from)?;
    let user_repository = Arc::new(UserRepository::new(user_data_source));

    let find_node = FindNode::default();
    let filter_users = FilterUsers::new(user_repository.clone());
    let user_credentials_verifier = UserCredentialsVerifier::default();
    let id_generator = GUIDGenerator::default();
    let create_user = CreateUser::new(
        user_repository.clone(),
        user_credentials_verifier,
        id_generator,
    );
    let update_user = UpdateUser::new(user_repository.clone());
    let delete_user = DeleteUser::new(user_repository);

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    let schema_builder = Schema::build(query, mutation, subscription)
        .register_output_type::<Node>()
        .data(find_node)
        .data(filter_users)
        .data(create_user)
        .data(update_user)
        .data(delete_user);
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
