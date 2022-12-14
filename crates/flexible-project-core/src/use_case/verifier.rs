//! Verifier use cases of the Flexible Project system.

#![allow(missing_docs)]

use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_more::{Display, Error, From, IsVariant, Unwrap};

use crate::model::user::{UserCredentials, UserToken, UserTokenClaims};

use super::error::InternalError;

/// Interactor type which can verify username provided by user.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait UsernameVerifier: Send + Sync {
    /// Verifies username provided by user.
    ///
    /// Returns `true` if provided username is valid, `false` otherwise.
    async fn verify(&self, username: String) -> Result<bool, InternalError>;
}

/// Interactor type which can verify password provided by user.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait PasswordVerifier: Send + Sync {
    /// Verifies password provided by user.
    ///
    /// Returns `true` if provided password is valid, `false` otherwise.
    async fn verify(&self, password: String) -> Result<bool, InternalError>;
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
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait UserCredentialsVerifier: Send + Sync {
    /// Verifies credentials provided by user.
    async fn verify(
        &self,
        credentials: UserCredentials,
    ) -> Result<UserCredentialsState, InternalError>;
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
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait UserTokenVerifier: Send + Sync {
    /// Verifies user token provided by client.
    ///
    /// Returns [token claims](UserTokenClaims) if provided token is valid.
    async fn verify(&self, token: UserToken) -> Result<UserTokenClaims, UserTokenError>;
}
