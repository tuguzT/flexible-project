//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject};
use fp_data::data_source::local::{Client, LocalUserDataSource};
use fp_data::interactor::hasher::PasswordHasher;
use fp_data::interactor::id::IdGenerator;
use fp_data::interactor::node::FindNode;
use fp_data::interactor::user::{
    CurrentUser, DeleteUser, FilterUsers, SignIn, SignUp, UserTokenGenerator,
};
use fp_data::interactor::verifier::{UserCredentialsVerifier, UserTokenVerifier};
use fp_data::repository::user::UserRepository;
use fp_data::repository::Error;

use crate::model::node::Node;

pub mod node;
pub mod user;

/// GraphQL schema of the Flexible Project system.
pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// Builder of the Flexible Project system [schema](Schema).
pub type SchemaBuilder = async_graphql::SchemaBuilder<Query, Mutation, EmptySubscription>;

/// Builds the [schema](Schema) for the Flexible Project system.
///
/// Returns a [builder](SchemaBuilder) to allow users to customize it.
pub async fn build_schema() -> Result<SchemaBuilder> {
    let client = Client::new().map_err(Error::from)?;
    let user_data_source = LocalUserDataSource::new(client)
        .await
        .map_err(Error::from)?;
    let user_repository = Arc::new(UserRepository::new(user_data_source));

    let filter_users = FilterUsers::new(user_repository.clone());
    let credentials_verifier = UserCredentialsVerifier::default();
    let id_generator = IdGenerator::default();
    let password_hasher = Arc::new(PasswordHasher::default());
    let token_generator = UserTokenGenerator::default();
    let sign_up = SignUp::new(
        user_repository.clone(),
        password_hasher.clone(),
        credentials_verifier.clone(),
        id_generator,
        token_generator.clone(),
    );
    let token_verifier = UserTokenVerifier::default();
    let sign_in = SignIn::new(
        user_repository.clone(),
        password_hasher,
        credentials_verifier,
        token_generator,
    );
    let current_user = CurrentUser::new(user_repository.clone(), token_verifier.clone());
    let delete_user = DeleteUser::new(user_repository, current_user.clone());
    let find_node = FindNode::new(filter_users.clone());

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    let schema_builder = Schema::build(query, mutation, subscription)
        .data(find_node)
        .data(filter_users)
        .data(sign_up)
        .data(sign_in)
        .data(current_user)
        .data(delete_user)
        .data(token_verifier)
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
