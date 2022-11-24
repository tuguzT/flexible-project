//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, MergedObject};
use fp_core::use_case::{
    node::FindNode as CoreFindNode,
    user::{
        CurrentUser as CoreCurrentUser, DeleteUser as CoreDeleteUser,
        FilterUsers as CoreFilterUsers, SignIn as CoreSignIn, SignUp as CoreSignUp,
    },
};
use fp_data::{
    data_source::{
        local::{Client, LocalUserDataSource},
        user::UserDataSource,
    },
    interactor::{
        hasher::PasswordHasher,
        id::IdGenerator,
        node::FindNode,
        user::{CurrentUser, DeleteUser, FilterUsers, SignIn, SignUp, UserTokenGenerator},
        verifier::{UserCredentialsVerifier, UserTokenVerifier},
    },
    repository::{user::UserRepository, Error},
};

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
    let user_data_source = Arc::new(user_data_source) as Arc<dyn UserDataSource>;
    let user_repository = UserRepository::new(user_data_source);

    let filter_users = Arc::new(FilterUsers::new(user_repository.clone()));
    let credentials_verifier = Arc::new(UserCredentialsVerifier::default());
    let id_generator = Arc::new(IdGenerator::default());
    let password_hasher = Arc::new(PasswordHasher::default());
    let token_generator = UserTokenGenerator::default();
    let sign_up = Arc::new(SignUp::new(
        user_repository.clone(),
        password_hasher.clone(),
        credentials_verifier.clone(),
        id_generator,
        token_generator.clone(),
    ));
    let token_verifier = Arc::new(UserTokenVerifier::default());
    let sign_in = Arc::new(SignIn::new(
        user_repository.clone(),
        password_hasher,
        credentials_verifier,
        token_generator,
    ));
    let current_user = Arc::new(CurrentUser::new(user_repository.clone(), token_verifier));
    let delete_user = Arc::new(DeleteUser::new(user_repository, current_user.clone()));
    let find_node = Arc::new(FindNode::new(filter_users.clone()));

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    let schema_builder = Schema::build(query, mutation, subscription)
        .data(find_node as Arc<dyn CoreFindNode>)
        .data(filter_users as Arc<dyn CoreFilterUsers>)
        .data(sign_up as Arc<dyn CoreSignUp>)
        .data(sign_in as Arc<dyn CoreSignIn>)
        .data(current_user as Arc<dyn CoreCurrentUser>)
        .data(delete_user as Arc<dyn CoreDeleteUser>)
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
