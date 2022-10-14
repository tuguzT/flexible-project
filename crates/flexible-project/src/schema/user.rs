//! Definitions of user queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Error, Object, ID};
use fp_core::model::UserFilters;
use fp_core::use_case::{CreateUser as _, DeleteUser as _, FilterUsers as _, UpdateUser as _};
use fp_data::data_source::local::LocalUserDataSource;
use fp_data::interactor::{
    CreateUser, DeleteUser, FilterUsers, UpdateUser as UpdateUserInteractor,
};

use crate::model::{UpdateUser, User, UserCredentials};

/// User query object of the Flexible Project system.
#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Data of all users of the Flexible Project system.
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>, Error> {
        let interactor = ctx
            .data::<FilterUsers<LocalUserDataSource>>()
            .expect("filter users interactor should always exist");
        let filters = UserFilters::default();
        let users = interactor.filter(filters).await?;
        Ok(users.into_iter().map(User::from).collect())
    }

    /// Data of user by its identifier of the Flexible Project system.
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: ID,
    ) -> Result<Option<User>, Error> {
        let interactor = ctx
            .data::<FilterUsers<LocalUserDataSource>>()
            .expect("filter users interactor should always exist");
        let id = id.to_string().into();
        let filters = UserFilters { ids: vec![id] };
        let user = interactor.filter(filters).await?.first().cloned();
        let user = user.map(User::from);
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
        #[graphql(desc = "User credentials of the new user.")] credentials: UserCredentials,
    ) -> Result<User, Error> {
        let interactor = ctx
            .data::<CreateUser<LocalUserDataSource>>()
            .expect("create user interactor should always exist");
        let user = interactor.create(credentials.into()).await?.into();
        Ok(user)
    }

    /// Updates existing user from provided user data in the Flexible Project system.
    async fn update_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "User data to be updated.")] user: UpdateUser,
    ) -> Result<Option<User>, Error> {
        let interactor = ctx
            .data::<UpdateUserInteractor<LocalUserDataSource>>()
            .expect("update user interactor should always exist");
        let user = interactor.update(user.into()).await?.map(User::from);
        Ok(user)
    }

    /// Deletes user by its identifier from the Flexible Project system.
    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: ID,
    ) -> Result<Option<User>, Error> {
        let interactor = ctx
            .data::<DeleteUser<LocalUserDataSource>>()
            .expect("delete user interactor should always exist");
        let id = id.to_string().into();
        let user = interactor.delete(id).await?.map(User::from);
        Ok(user)
    }
}
