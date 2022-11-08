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
    WithSecret(WithSecretError),
    /// Password hash error variant.
    PasswordHash(PasswordHashError),
    /// Password hash verification error variant.
    PasswordHashVerify(PasswordHashVerifyError),
    /// Sign in error variant.
    SignIn(SignInError),
    /// Sign up error variant.
    SignUp(SignUpError),
    /// Repository error variant.
    Repository(RepositoryError),
}
