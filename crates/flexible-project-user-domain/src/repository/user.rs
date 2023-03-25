use async_trait::async_trait;
use auto_impl::auto_impl;
use futures::Stream;

use crate::model::{User, UserData, UserFilters, UserId};

/// Database of user microservice data.
#[async_trait(?Send)]
#[auto_impl(&, Box, Arc)]
pub trait UserDatabase {
    /// The type returned when a repository fails to apply an operation.
    type Error;

    /// Creates new user from provided identifier and user data.
    ///
    /// Returns new user or an error if user with such identifier already exists.
    async fn create(&self, id: UserId, data: UserData) -> Result<User, Self::Error>;

    /// Type of stream which produces filtered repository data.
    type Users: Stream<Item = Result<User, Self::Error>>;
    /// Filters users by provided filter object.
    async fn read(&self, filter: UserFilters<'_>) -> Result<Self::Users, Self::Error>;

    /// Updates user by provided identifier with provided data.
    ///
    /// Returns updated user or an error if user with such identifier does not exist.
    async fn update(&self, id: UserId, data: UserData) -> Result<User, Self::Error>;

    /// Deletes user from the repository by provided identifier.
    ///
    /// Returns deleted user or an error if user with such identifier does not exist.
    async fn delete(&self, id: UserId) -> Result<User, Self::Error>;
}
