//! Definitions of user queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Object, Result, ID};
use fp_core::model::id::{Id, IdFilters};
use fp_core::model::user::UserFilters as CoreUserFilters;

use crate::model::user::{User, UserCredentials, UserFilters, UserToken};

use super::di::interactor::user::{CurrentUser, DeleteUser, FilterUsers, SignIn, SignUp};
use super::ext::ContextExt;

/// User query object of the Flexible Project system.
#[derive(Debug, Default)]
pub struct UserQuery(());

#[Object]
impl UserQuery {
    /// Data of all users of the Flexible Project system.
    async fn users(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "User filters.", default_with = "UserFilters::default()")]
        filters: UserFilters,
    ) -> Result<Vec<User>> {
        let interactor = ctx.resolve_ref::<dyn FilterUsers>();
        let filters = filters.into();
        let users = interactor.filter(filters).await?;
        let users = users.into_iter().map(User::from).collect();
        Ok(users)
    }

    /// Data of user by its identifier of the Flexible Project system.
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: ID,
    ) -> Result<Option<User>> {
        let interactor = ctx.resolve_ref::<dyn FilterUsers>();
        let id = Id::from(id.to_string());
        let filters = CoreUserFilters::builder()
            .id(IdFilters::builder().eq(id).build())
            .build();
        let user = interactor.filter(filters).await?.first().cloned();
        let user = user.map(User::from);
        Ok(user)
    }

    /// Data of the current user of the Flexible project system.
    async fn current_user(&self, ctx: &Context<'_>) -> Result<User> {
        let token = require_user_token(ctx)?.into();
        let interactor = ctx.resolve_ref::<dyn CurrentUser>();
        let user = interactor.current_user(token).await?.into();
        Ok(user)
    }
}

/// User mutation object of the Flexible Project system.
#[derive(Debug, Default)]
pub struct UserMutation(());

#[Object]
impl UserMutation {
    /// Registers new user in the Flexible Project system.
    async fn sign_up(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "User credentials of the new user.")] credentials: UserCredentials,
    ) -> Result<UserToken> {
        let interactor = ctx.resolve_ref::<dyn SignUp>();
        let token = interactor.sign_up(credentials.into()).await?.into();
        Ok(token)
    }

    /// Login existing user in the Flexible Project system.
    async fn sign_in(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "User credentials of the existing user.")] credentials: UserCredentials,
    ) -> Result<UserToken> {
        let interactor = ctx.resolve_ref::<dyn SignIn>();
        let token = interactor.sign_in(credentials.into()).await?.into();
        Ok(token)
    }

    /// Deletes user by its identifier from the Flexible Project system.
    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: ID,
    ) -> Result<Option<User>> {
        let token = require_user_token(ctx)?.into();
        let interactor = ctx.resolve_ref::<dyn DeleteUser>();
        let id = id.to_string().into();
        let user = interactor.delete(token, id).await?.map(User::from);
        Ok(user)
    }
}

/// Tries to retrieve [user token](UserToken) from the GraphQL [context](Context).
pub fn require_user_token(ctx: &Context<'_>) -> Result<UserToken> {
    let token = ctx
        .data_opt::<UserToken>()
        .cloned()
        .ok_or("authentication is required")?;
    Ok(token)
}
