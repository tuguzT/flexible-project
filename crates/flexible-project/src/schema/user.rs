//! Definitions of user queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Error, Object, ID};
use fp_core::model::UserFilters;
use fp_core::use_case::{
    DeleteUser as _, FilterUsers as _, SignIn as _, SignUp as _, UpdateUser as _,
    UserTokenVerifier as _,
};
use fp_data::data_source::local::LocalUserDataSource;
use fp_data::interactor::{
    DeleteUser, FilterUsers, SignIn, SignUp, UpdateUser as UpdateUserInteractor, UserTokenVerifier,
};

use crate::model::{UpdateUser, User, UserCredentials, UserToken};

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
        let users = users.into_iter().map(User::from).collect();
        Ok(users)
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
    /// Registers new user in the Flexible Project system.
    async fn sign_up(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "User credentials of the new user.")] credentials: UserCredentials,
    ) -> Result<UserToken, Error> {
        let interactor = ctx
            .data::<SignUp<LocalUserDataSource>>()
            .expect("sign up interactor should always exist");
        let token = interactor.sign_up(credentials.into()).await?.into();
        Ok(token)
    }

    /// Login existing user in the Flexible Project system.
    async fn sign_in(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "User credentials of the existing user.")] credentials: UserCredentials,
    ) -> Result<User, Error> {
        let token = require_user_token(ctx)?;
        let interactor = ctx
            .data::<SignIn<LocalUserDataSource>>()
            .expect("sign in interactor should always exist");
        let user = interactor
            .sign_in(credentials.into(), token.into())
            .await?
            .into();
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

/// Tries to retrieve [user token](UserToken) from the GraphQL [context](Context).
pub fn require_user_token(ctx: &Context<'_>) -> Result<UserToken, Error> {
    let token = ctx
        .data_opt::<UserToken>()
        .cloned()
        .ok_or("authentication is required")?;
    Ok(token)
}

/// Tries to retrieve [user](User) from the GraphQL [context](Context).
pub async fn require_user(ctx: &Context<'_>) -> Result<User, Error> {
    let token = &require_user_token(ctx)?.into();
    let token_verifier = ctx
        .data::<UserTokenVerifier>()
        .expect("token verifier interactor should always exist");
    let claims = token_verifier.verify(token)?;

    let filter_users = ctx
        .data::<FilterUsers<LocalUserDataSource>>()
        .expect("filter users interactor should always exist");
    let id = claims.id;
    let filters = UserFilters { ids: vec![id] };
    let user = filter_users
        .filter(filters)
        .await?
        .first()
        .cloned()
        .ok_or("user is required")?
        .into();
    Ok(user)
}
