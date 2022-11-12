//! Verifier use cases of the Flexible Project system.

#![allow(missing_docs)]

use derive_more::{Display, Error, From, IsVariant, Unwrap};

use crate::model::user::{UserCredentials, UserToken, UserTokenClaims};
use crate::use_case::error::InternalError;

/// Interactor type which can verify username provided by user.
pub trait UsernameVerifier {
    /// Verifies username provided by user.
    ///
    /// Returns `true` if provided username is valid, `false` otherwise.
    fn verify(&self, username: &str) -> Result<bool, InternalError>;
}

/// Interactor type which can verify password provided by user.
pub trait PasswordVerifier {
    /// Verifies password provided by user.
    ///
    /// Returns `true` if provided password is valid, `false` otherwise.
    fn verify(&self, password: &str) -> Result<bool, InternalError>;
}

/// State of [user credentials](UserCredentials) after its checking by
/// [user credentials verifier](UserCredentialsVerifier).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, IsVariant, Unwrap)]
pub enum UserCredentialsState {
    /// [User credentials](UserCredentials) are totally valid.
    Valid,
    /// [User credentials](UserCredentials) name is invalid.
    InvalidUsername,
    /// [User credentials](UserCredentials) password is invalid.
    InvalidPassword,
}

/// Interactor type which can verify credentials provided by user.
pub trait UserCredentialsVerifier {
    /// Verifies credentials provided by user.
    fn verify(&self, credentials: &UserCredentials) -> Result<UserCredentialsState, InternalError>;
}

/// Error type of [token verifier](UserTokenVerifier) use case.
#[derive(Debug, Display, From, Error)]
pub enum UserTokenError {
    /// User token expired and needs to be updated.
    #[display(fmt = "user token was expired")]
    Expired,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can verify user token provided by client.
pub trait UserTokenVerifier {
    /// Verifies user token provided by client.
    ///
    /// Returns [token claims](UserTokenClaims) if provided token is valid.
    fn verify(&self, token: &UserToken) -> Result<UserTokenClaims, UserTokenError>;
}
