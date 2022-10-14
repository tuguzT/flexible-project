use derive_more::{Display, Error, From};

use crate::interactor::hasher::{PasswordHashError, PasswordHashVerifyError, WithSecretError};
use crate::interactor::user::CreateUserError;
use crate::repository::Error as RepositoryError;

/// Result of interactor operations with error type of [`struct@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of interactor implementations.
#[derive(Debug, Display, Error, From)]
#[from(forward)]
pub struct Error(#[error(source)] ErrorKind);

#[derive(Debug, Display, Error, From)]
#[display(fmt = "interactor error")]
enum ErrorKind {
    WithSecret(#[error(source)] WithSecretError),
    PasswordHash(#[error(source)] PasswordHashError),
    PasswordHashVerify(#[error(source)] PasswordHashVerifyError),
    CreateUser(#[error(source)] CreateUserError),
    Repository(#[error(source)] RepositoryError),
}
