//! Definitions of user queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Object, ID};
use fp_data::model::Id;
use fp_data::repository::ops::{DeleteById, ReadAll, ReadById, Save};
use uuid::Uuid;

use crate::data::UserRepositoryData;
use crate::model::{NewUser, User, UserRole};

/// User query object of the Flexible Project system.
#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Data of all users of the Flexible Project system.
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let repository = repository.read().await;
        let users = repository.read_all().await;
        users.into_iter().map(User::from).collect()
    }

    /// Data of user by its identifier of the Flexible Project system.
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: ID,
    ) -> Result<Option<User>, uuid::Error> {
        let id = Uuid::try_from(id)?.into();
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let repository = repository.read().await;
        let user = repository.read_by_id(id).await.map(User::from);
        Ok(user)
    }
}

/// User mutation object of the Flexible Project system.
#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Creates new user from provided user data in the Flexible Project system.
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Data of new user.")] user: NewUser,
    ) -> User {
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let mut repository = repository.write().await;
        let user = User {
            id: Id::random(),
            name: user.name,
            email: user.email,
            role: UserRole::User,
        };
        let user = repository.save(user.into()).await;
        User::from(user)
    }

    /// Deletes user by its identifier from the Flexible Project system.
    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: ID,
    ) -> Result<Option<User>, uuid::Error> {
        let id = Uuid::try_from(id)?.into();
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let mut repository = repository.write().await;
        let user = repository.delete_by_id(id).await.map(User::from);
        Ok(user)
    }
}
