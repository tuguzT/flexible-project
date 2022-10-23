use derive_more::{Display, Error, From};

use crate::interactor::hasher::{PasswordHashError, PasswordHashVerifyError, WithSecretError};
use crate::interactor::user::{SignInError, SignUpError};
use crate::repository::Error as RepositoryError;

/// Result of interactor operations with error type of [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of interactor implementations.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    /// With secret error variant.
    WithSecret(#[error(source)] WithSecretError),
    /// Password hash error variant.
    PasswordHash(#[error(source)] PasswordHashError),
    /// Password hash verification error variant.
    PasswordHashVerify(#[error(source)] PasswordHashVerifyError),
    /// Sign in error variant.
    SignIn(#[error(source)] SignInError),
    /// Sign up error variant.
    SignUp(#[error(source)] SignUpError),
    /// Repository error variant.
    Repository(#[error(source)] RepositoryError),
}
