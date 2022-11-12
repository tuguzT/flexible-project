//! Hashing use cases of the Flexible Project system.

use async_trait::async_trait;

use crate::use_case::error::InternalError;

/// Interactor type which can hash password with some algorithm.
#[async_trait]
pub trait PasswordHasher {
    /// Hashes provided password with some algorithm.
    ///
    /// Returns [`String`] with hashed password.
    async fn hash(&self, password: String) -> Result<String, InternalError>;
}

/// Interactor type which can verify password by its hash.
#[async_trait]
pub trait PasswordHashVerifier {
    /// Verifies provided password with its hash.
    ///
    /// Returns `true` if the hash of provided password and
    /// provided password hash are equal, `false` otherwise.
    async fn verify(&self, password: String, password_hash: String) -> Result<bool, InternalError>;
}
