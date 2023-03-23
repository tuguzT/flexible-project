use std::{borrow::Borrow, pin::pin};

use async_trait::async_trait;
use auto_impl::auto_impl;
use fp_core::filter::Borrowed;
use futures::{Stream, StreamExt, TryStreamExt};

use crate::model::{Name, NameFilters, User, UserData, UserFilters, UserId, UserIdFilters};

/// Defines operations applicable to the user microservice data.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait Repository {
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

pub async fn find_one_by_id<R, Id>(repository: R, id: Id) -> Result<Option<User>, R::Error>
where
    R: Repository,
    Id: Borrow<UserId>,
{
    let id = id.borrow();
    let filter = UserFilters::builder()
        .id(UserIdFilters::builder().eq(id.borrowed()).build())
        .build();
    let users = repository.read(filter).await?;
    let mut users = pin!(users);
    let user = users.try_next().await?;
    debug_assert!(
        users.count().await == 0,
        "exactly one user should present with id {id}",
    );
    Ok(user)
}

pub async fn find_one_by_name<R, N>(repository: R, name: N) -> Result<Option<User>, R::Error>
where
    R: Repository,
    N: Borrow<Name>,
{
    let name = name.borrow();
    let filter = UserFilters::builder()
        .name(NameFilters::builder().eq(name.borrowed()).build())
        .build();
    let users = repository.read(filter).await?;
    let mut users = pin!(users);
    let user = users.try_next().await?;
    debug_assert!(
        users.count().await == 0,
        "exactly one user should present with name {name}",
    );
    Ok(user)
}
