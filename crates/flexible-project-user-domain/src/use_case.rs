//! Use cases of the user microservice domain layer.

use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_more::{Display, Error, From};
use fp_core::filter::{Borrowed, Owned};

use crate::model::{Name, NameFilters, User, UserData, UserFilters, UserId, UserIdFilters};

/// Defines operations applicable to the user microservice data.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait Repository {
    /// The type returned when a repository fails to apply an operation.
    type Error;

    /// Create new user from provided identifier and user data.
    ///
    /// Returns new user or an error if user with such identifier already exists.
    async fn create(&self, id: UserId, data: UserData) -> Result<User, Self::Error>;

    /// Type of iterator of filtered repository data.
    type Users: IntoIterator<Item = User>;
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

/// Error type of update user name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    AlreadyTaken,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates name of the user by its identifier with provided name.
pub async fn update_name<R>(
    repository: R,
    id: UserId,
    name: Name,
) -> Result<User, UpdateNameError<R::Error>>
where
    R: Repository,
{
    let is_name_unique = {
        let filter = UserFilters::builder()
            .name(NameFilters::builder().eq(name.borrowed()).build())
            .build();
        let users = repository.read(filter).await?;
        users.into_iter().count() == 0
    };
    if !is_name_unique {
        return Err(UpdateNameError::AlreadyTaken);
    }

    let User { id, data } = {
        let filter = UserFilters::builder()
            .id(UserIdFilters::builder().eq(id.owned()).build())
            .build();
        let users = repository.read(filter).await?;
        let mut users = users.into_iter();
        users.next().ok_or(UpdateNameError::NoUser)?
    };
    let data = UserData { name, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Filters users by provided filter object.
pub async fn filter_users<R>(repository: R, filter: UserFilters<'_>) -> Result<R::Users, R::Error>
where
    R: Repository,
{
    repository.read(filter).await
}
