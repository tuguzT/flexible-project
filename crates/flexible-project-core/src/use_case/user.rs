use async_trait::async_trait;

use crate::model::{Identifiable, User, UserCredentials, UserFilters};

/// Trait for interactors which create new user from provided user credentials.
#[async_trait]
pub trait CreateUser {
    /// Output user data.
    type User: User;

    /// The type returned when any error occurs.
    type Error;

    /// Creates new user from provided user credentials.
    ///
    /// Returns data of created user.
    async fn create<C>(&self, credentials: &C) -> Result<Self::User, Self::Error>
    where
        C: UserCredentials + Sync;
}

/// Trait for interactors which filter all the users.
#[async_trait]
pub trait FilterUsers {
    /// User data type of the implementation.
    type User: User;

    /// The type returned when any error occurs.
    type Error;

    /// Filters all the users with provided [filters](UserFilters).
    ///
    /// Returns [`Vec`] with filter results.
    async fn filter(&self, filters: UserFilters) -> Result<Vec<Self::User>, Self::Error>;
}

/// Trait for interactors which update user state.
#[async_trait]
pub trait UpdateUser {
    /// User data type of the implementation.
    type User: User;

    /// The type returned when any error occurs.
    type Error;

    /// Updates user state in the system.
    ///
    /// Returns updated user data.
    async fn update(&self, user: Self::User) -> Result<Self::User, Self::Error>;
}

/// Trait for interactors which delete user from the system.
#[async_trait]
pub trait DeleteUser {
    /// User data type of the implementation.
    type User: User;

    /// The type returned when any error occurs.
    type Error;

    /// Deletes the user with provided identifier.
    ///
    /// Returns data of the deleted user.
    async fn delete(&self, id: <Self::User as Identifiable>::Id)
        -> Result<Self::User, Self::Error>;
}
