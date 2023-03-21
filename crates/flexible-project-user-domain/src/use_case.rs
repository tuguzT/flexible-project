//! Use cases of the user microservice domain layer.

use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_more::{Display, Error, From};
use fancy_regex::Regex;
use once_cell::sync::Lazy;

use crate::model::{NameFilters, User, UserData, UserFilters, UserId, UserIdFilters};

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
    async fn read(&self, filter: UserFilters) -> Result<Self::Users, Self::Error>;

    /// Updates user by provided identifier with provided data.
    ///
    /// Returns updated user or an error if user with such identifier does not exist.
    async fn update(&self, id: UserId, data: UserData) -> Result<User, Self::Error>;

    /// Deletes user from the repository by provided identifier.
    ///
    /// Returns deleted user or an error if user with such identifier does not exist.
    async fn delete(&self, id: UserId) -> Result<User, Self::Error>;
}

/// Checks if user name meets all the requirements.
///
/// These requirements are:
/// - must be from 4 to 32 characters in length;
/// - must contain latin or `-`, `_`, `.` characters;
/// - must not start or end with `-`, `_`, `.` characters;
/// - `-`, `_`, `.` characters can't be next to each other;
/// - `-`, `_`, `.` characters can't be used multiple times in a row.
pub fn verify_name(name: &str) -> bool {
    static NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?=.{4,32}$)(?![-_.])(?!.*[-_.]{2})[a-zA-Z\d\-_.]+(?<![-_.])$").unwrap()
    });

    NAME_REGEX.is_match(name).expect("regex should be valid")
}

/// Error type of update user name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// Invalid user name was provided.
    #[display(fmt = "username does not match requirements")]
    InvalidName,
    /// User with provided name already exists.
    #[display(fmt = "user name already taken")]
    AlreadyTaken,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates name of the user by its identifier with provided name.
pub async fn update_name<R>(
    repository: R,
    id: UserId,
    name: String,
) -> Result<User, UpdateNameError<R::Error>>
where
    R: Repository,
{
    if verify_name(&name) {
        return Err(UpdateNameError::InvalidName);
    }
    let is_name_unique = {
        let filter = UserFilters::builder()
            .name(NameFilters::builder().eq(name.clone()).build())
            .build();
        let users = repository.read(filter).await?;
        users.into_iter().count() == 0
    };
    if !is_name_unique {
        return Err(UpdateNameError::AlreadyTaken);
    }

    let User { id, data } = {
        let filter = UserFilters::builder()
            .id(UserIdFilters::builder().eq(id).build())
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
pub async fn filter_users<R>(repository: R, filter: UserFilters) -> Result<R::Users, R::Error>
where
    R: Repository,
{
    repository.read(filter).await
}
