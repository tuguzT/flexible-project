//! Hashing use cases of the Flexible Project system.

use async_trait::async_trait;
use auto_impl::auto_impl;

use super::error::InternalError;

/// Interactor type which can hash password with some algorithm.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait PasswordHasher: Send + Sync {
    /// Hashes provided password with some algorithm.
    ///
    /// Returns [`String`] with hashed password.
    async fn hash(&self, password: String) -> Result<String, InternalError>;
}

/// Interactor type which can verify password by its hash.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait PasswordHashVerifier: Send + Sync {
    /// Verifies provided password with its hash.
    ///
    /// Returns `true` if the hash of provided password and
    /// provided password hash are equal, `false` otherwise.
    async fn verify(&self, password: String, password_hash: String) -> Result<bool, InternalError>;
}
