use async_trait::async_trait;

use crate::model::{Id, User, UserCredentials, UserFilters, UserToken, UserTokenClaims};

/// Interactor type which can generate new user token from the claims.
pub trait UserTokenGenerator {
    /// The type returned when any error occurs.
    type Error;

    /// Generates new [user token](UserToken) with data provided in [claims](UserTokenClaims).
    fn generate(&self, claims: UserTokenClaims) -> Result<UserToken, Self::Error>;
}

/// Interactor type which can register new user.
#[async_trait]
pub trait SignUp {
    /// The type returned when any error occurs.
    type Error;

    /// Registers new user from provided [credentials](UserCredentials)
    /// in the Flexible Project system.
    async fn sign_up(&self, credentials: UserCredentials) -> Result<UserToken, Self::Error>;
}

/// Interactor type which can login existing user.
#[async_trait]
pub trait SignIn {
    /// The type returned when any error occurs.
    type Error;

    /// Login existing user with provided [credentials](UserCredentials)
    /// and [user token](UserToken) in the Flexible Project system.
    async fn sign_in(
        &self,
        credentials: UserCredentials,
        token: UserToken,
    ) -> Result<User, Self::Error>;
}

/// Interactor type which can filter all the users.
#[async_trait]
pub trait FilterUsers {
    /// The type returned when any error occurs.
    type Error;

    /// Filters all the users with provided [filters](UserFilters).
    ///
    /// Returns [`Vec`] with filter results.
    async fn filter(&self, filters: UserFilters) -> Result<Vec<User>, Self::Error>;
}

/// Interactor type which can update user state.
#[async_trait]
pub trait UpdateUser {
    /// The type returned when any error occurs.
    type Error;

    /// Updates user state in the system.
    ///
    /// Returns updated user data.
    async fn update(&self, user: User) -> Result<Option<User>, Self::Error>;
}

/// Interactor type which can delete user from the system.
#[async_trait]
pub trait DeleteUser {
    /// The type returned when any error occurs.
    type Error;

    /// Deletes the user with provided identifier.
    ///
    /// Returns data of the deleted user if present.
    async fn delete(&self, id: Id<User>) -> Result<Option<User>, Self::Error>;
}
