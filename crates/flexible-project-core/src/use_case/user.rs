use async_trait::async_trait;

use crate::model::{Id, User, UserCredentials, UserFilters};

/// Interactor type which can create new user from provided user credentials.
#[async_trait]
pub trait CreateUser {
    /// The type returned when any error occurs.
    type Error;

    /// Creates new user from provided user credentials.
    ///
    /// Returns data of created user.
    async fn create(&self, credentials: UserCredentials) -> Result<User, Self::Error>;
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
