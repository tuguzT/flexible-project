//! Use cases of the user microservice domain layer.

use std::{borrow::Borrow, pin::pin};

use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_more::{Display, Error, From};
use fp_core::filter::Borrowed;
use futures::{Stream, StreamExt, TryStreamExt};

use crate::model::{
    DisplayName, Email, Name, NameFilters, Role, User, UserData, UserFilters, UserId, UserIdFilters,
};

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

/// Error type of create user use case.
#[derive(Debug, Display, From, Error)]
pub enum CreateUserError<Error> {
    /// User with provided identifier already exists.
    #[display(fmt = "identifier is already taken")]
    IdAlreadyTaken,
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    NameAlreadyTaken,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Creates new user from provided identifier and user data.
pub async fn create_user<R>(
    repository: R,
    id: UserId,
    data: UserData,
) -> Result<User, CreateUserError<R::Error>>
where
    R: Repository,
{
    let id_exists = {
        let user_by_id = find_one_by_id(&repository, &id).await?;
        user_by_id.is_some()
    };
    if id_exists {
        return Err(CreateUserError::IdAlreadyTaken);
    }

    let UserData { ref name, .. } = data;
    let is_name_unique = {
        let user_by_name = find_one_by_name(&repository, name).await?;
        user_by_name.is_none()
    };
    if !is_name_unique {
        return Err(CreateUserError::NameAlreadyTaken);
    }

    let user = repository.create(id, data).await?;
    Ok(user)
}

/// Filters users by provided filter object.
pub async fn filter_users<R>(repository: R, filter: UserFilters<'_>) -> Result<R::Users, R::Error>
where
    R: Repository,
{
    repository.read(filter).await
}

async fn find_one_by_id<R, Id>(repository: R, id: Id) -> Result<Option<User>, R::Error>
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

async fn find_one_by_name<R, N>(repository: R, name: N) -> Result<Option<User>, R::Error>
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
        let user_by_name = find_one_by_name(&repository, &name).await?;
        user_by_name.is_none()
    };
    if !is_name_unique {
        return Err(UpdateNameError::AlreadyTaken);
    }

    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateNameError::NoUser)?
    };
    let data = UserData { name, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of update user display name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateDisplayNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates display name of the user by its identifier with provided display name.
pub async fn update_display_name<R>(
    repository: R,
    id: UserId,
    display_name: DisplayName,
) -> Result<User, UpdateDisplayNameError<R::Error>>
where
    R: Repository,
{
    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateDisplayNameError::NoUser)?
    };
    let data = UserData {
        display_name,
        ..data
    };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of update user role use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateRoleError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates role of the user by its identifier with provided role.
pub async fn update_role<R>(
    repository: R,
    id: UserId,
    role: Role,
) -> Result<User, UpdateRoleError<R::Error>>
where
    R: Repository,
{
    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateRoleError::NoUser)?
    };
    let data = UserData { role, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of update user email use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateEmailError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates email of the user by its identifier with provided email.
pub async fn update_email<R>(
    repository: R,
    id: UserId,
    email: Option<Email>,
) -> Result<User, UpdateEmailError<R::Error>>
where
    R: Repository,
{
    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateEmailError::NoUser)?
    };
    let data = UserData { email, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of delete user use case.
#[derive(Debug, Display, From, Error)]
pub enum DeleteUserError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Deletes user by provided identifier.
pub async fn delete_user<R>(repository: R, id: UserId) -> Result<User, DeleteUserError<R::Error>>
where
    R: Repository,
{
    let id_exists = {
        let user_by_id = find_one_by_id(&repository, &id).await?;
        user_by_id.is_some()
    };
    if !id_exists {
        return Err(DeleteUserError::NoUser);
    }

    let user = repository.delete(id).await?;
    Ok(user)
}
