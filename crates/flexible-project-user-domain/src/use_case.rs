//! Use cases of the user microservice domain layer.

use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_more::{Display, Error, From};
use fancy_regex::Regex;
use once_cell::sync::Lazy;

use crate::model::{User, UserData, UserId};

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
    type Users: Iterator<Item = User>;
    /// Filter users by provided filter object.
    // TODO replace with actual filter type
    async fn read(&self, filter: UserId) -> Result<Self::Users, Self::Error>;

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

/// Checks if user name meets all the requirements.
///
/// These requirements are:
/// - must be from 4 to 32 characters in length;
/// - must contain latin or `-`, `_`, `.` characters;
/// - must not start or end with `-`, `_`, `.` characters;
/// - `-`, `_`, `.` characters can't be next to each other;
/// - `-`, `_`, `.` characters can't be used multiple times in a row.
pub fn verify_name(name: &str) -> bool {
    static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?=.{4,32}$)(?![-_.])(?!.*[-_.]{2})[a-zA-Z\d\-_.]+(?<![-_.])$").unwrap()
    });

    USERNAME_REGEX
        .is_match(name)
        .expect("regex should be valid")
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
    // TODO check if user with such name does not exist
    let is_name_unique = true;
    if !is_name_unique {
        return Err(UpdateNameError::AlreadyTaken);
    }

    let User { id, data } = {
        let mut users = repository.read(id).await?;
        users.next().ok_or(UpdateNameError::NoUser)?
    };
    let data = UserData { name, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}
