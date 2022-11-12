//! User use cases of the Flexible Project system.

use async_trait::async_trait;
use derive_more::{Display, Error, From};

use crate::model::id::Id;
use crate::model::user::{
    User, UserCredentials, UserFilters, UserRole, UserToken, UserTokenClaims,
};
use crate::use_case::error::InternalError;
use crate::use_case::verifier::UserTokenError;

/// Interactor type which can generate new user token from the claims.
pub trait UserTokenGenerator {
    /// Generates new [user token](UserToken) with data provided in [claims](UserTokenClaims).
    fn generate(&self, claims: UserTokenClaims) -> Result<UserToken, InternalError>;
}

/// Error type of [sign up](SignUp) use case.
#[derive(Debug, Display, From, Error)]
pub enum SignUpError {
    /// Invalid username was provided.
    #[display(fmt = "provided username does not match requirements")]
    InvalidUsername,
    /// Invalid password was provided.
    #[display(fmt = "provided password does not match requirements")]
    InvalidPassword,
    /// User with provided username already registered in the Flexible project system.
    #[display(fmt = "user name already taken")]
    UsernameAlreadyTaken,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can register new user.
#[async_trait]
pub trait SignUp {
    /// Registers new user from provided [credentials](UserCredentials)
    /// in the Flexible Project system.
    async fn sign_up(&self, credentials: UserCredentials) -> Result<UserToken, SignUpError>;
}

/// Error type of [sign in](SignIn) use case.
#[derive(Debug, Display, From, Error)]
pub enum SignInError {
    /// Invalid username was provided.
    #[display(fmt = "username does not match requirements")]
    InvalidUsername,
    /// Invalid password was provided.
    #[display(fmt = "password does not match requirements")]
    InvalidPassword,
    /// Provided password does not match actual user password.
    #[display(fmt = "password does not match actual user password")]
    WrongPassword,
    /// No user was found by provided username.
    #[display(fmt = "no user was found by username")]
    NoUser,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can login existing user.
#[async_trait]
pub trait SignIn {
    /// Login existing user with provided [credentials](UserCredentials)
    /// in the Flexible Project system.
    async fn sign_in(&self, credentials: UserCredentials) -> Result<UserToken, SignInError>;
}

/// Interactor type which can filter all users of the system.
#[async_trait]
pub trait FilterUsers {
    /// Filters all users with provided [filters](UserFilters).
    ///
    /// Returns collection of filter results.
    async fn filter(&self, filters: UserFilters) -> Result<Vec<User>, InternalError>;
}

/// Error type of [current user](CurrentUser) use case.
#[derive(Debug, Display, From, Error)]
pub enum CurrentUserError {
    /// User token error.
    UserToken(UserTokenError),
    /// No user was found by this token.
    #[display(fmt = "no user by token")]
    NoUser,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can get current user data by [token](UserToken).
#[async_trait]
pub trait CurrentUser {
    /// Get data of current user by provided token.
    async fn current_user(&self, token: UserToken) -> Result<User, CurrentUserError>;
}

/// Error type of [update user name](UpdateUsername) use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateUsernameError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// Invalid username was provided.
    #[display(fmt = "username does not match requirements")]
    InvalidUsername,
    /// User with provided username already registered in the Flexible project system.
    #[display(fmt = "user name already taken")]
    AlreadyTaken,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can update current user name.
#[async_trait]
pub trait UpdateUsername {
    /// Updates current user name with provided name.
    async fn update_name(
        &self,
        token: UserToken,
        name: String,
    ) -> Result<User, UpdateUsernameError>;
}

/// Error type of [update user password](UpdateUserPassword) use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateUserPasswordError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// Invalid password was provided.
    #[display(fmt = "password does not match requirements")]
    InvalidPassword,
    /// Provided password does not match actual user password.
    #[display(fmt = "old password does not match actual user password")]
    WrongPassword,
    /// New provided password is the same as old password.
    #[display(fmt = "new password is the same as actual password")]
    SamePassword,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can update current user password.
#[async_trait]
pub trait UpdateUserPassword {
    /// Updates current user password with provided new password.
    async fn update_password(
        &self,
        token: UserToken,
        old_password: String,
        new_password: String,
    ) -> Result<(), UpdateUserPasswordError>;
}

/// Error type of [update user display name](UpdateUserDisplayName) use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateUserDisplayNameError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can update current user display name.
#[async_trait]
pub trait UpdateUserDisplayName {
    /// Updates current user display name with provided name.
    async fn update_display_name(
        &self,
        token: UserToken,
        display_name: String,
    ) -> Result<User, UpdateUserDisplayNameError>;
}

/// Error type of [update user email](UpdateUserEmail) use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateUserEmailError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// Provided email already taken by another user.
    #[display(fmt = "user email already taken")]
    AlreadyTaken,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can update current user email.
#[async_trait]
pub trait UpdateUserEmail {
    /// Updates current user display name with provided name.
    async fn update_email(
        &self,
        token: UserToken,
        email: Option<String>,
    ) -> Result<User, UpdateUserEmailError>;
}

/// Error type of [grant user role](GrantUserRole) use case.
#[derive(Debug, Display, From, Error)]
pub enum GrantUserRoleError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// User does not allowed to grant role to another user.
    #[display(fmt = "not allowed to grant role to another user")]
    NotAllowed,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can update role of another user.
#[async_trait]
pub trait GrantUserRole {
    /// Updates role of another user
    /// if current user is [administrator](UserRole::Administrator).
    async fn grant_role(
        &self,
        token: UserToken,
        user_to_grant: Id<User>,
        role: UserRole,
    ) -> Result<(), GrantUserRoleError>;
}

/// Error type of [delete user](DeleteUser) use case.
#[derive(Debug, Display, From, Error)]
pub enum DeleteUserError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// User does not allowed to delete another user.
    #[display(fmt = "not allowed to delete another user")]
    NotAllowed,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can delete user from the system.
#[async_trait]
pub trait DeleteUser {
    /// Deletes the user with provided identifier.
    ///
    /// Returns data of the deleted user if present.
    async fn delete(
        &self,
        token: UserToken,
        user_to_delete: Id<User>,
    ) -> Result<Option<User>, DeleteUserError>;
}
