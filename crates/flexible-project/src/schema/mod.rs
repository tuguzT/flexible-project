//! Definitions of GraphQL schemas, queries, mutations and subscriptions.

use std::sync::Arc;

use async_graphql::{EmptySubscription, MergedObject};
use fp_data::data_source::local::{Client, LocalUserDataSource};
use fp_data::interactor::{
    DeleteUser, FilterUsers, FindNode, GUIDGenerator, PasswordHasher, SignIn, SignUp, UpdateUser,
    UserCredentialsVerifier, UserTokenGenerator, UserTokenVerifier,
};
use fp_data::repository::user::UserRepository;
use fp_data::Result;

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
    let client = Client::new()?;
    let database = client.0.database("flexible-project");
    let user_data_source = LocalUserDataSource::new(database).await?;
    let user_repository = Arc::new(UserRepository::new(user_data_source));

    let find_node = FindNode::new(user_repository.clone());
    let filter_users = FilterUsers::new(user_repository.clone());
    let credentials_verifier = UserCredentialsVerifier::default();
    let id_generator = GUIDGenerator::default();
    let password_hasher = Arc::new(PasswordHasher::default());
    let token_generator = UserTokenGenerator::default();
    let sign_up = SignUp::new(
        user_repository.clone(),
        password_hasher.clone(),
        credentials_verifier,
        id_generator,
        token_generator,
    );
    let token_verifier = UserTokenVerifier::default();
    let sign_in = SignIn::new(
        user_repository.clone(),
        password_hasher,
        credentials_verifier,
        token_generator,
    );
    let update_user = UpdateUser::new(user_repository.clone());
    let delete_user = DeleteUser::new(user_repository);

    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    let schema_builder = Schema::build(query, mutation, subscription)
        .data(find_node)
        .data(filter_users)
        .data(sign_up)
        .data(sign_in)
        .data(update_user)
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
