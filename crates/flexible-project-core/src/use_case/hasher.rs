//! Hashing use cases of the Flexible Project system.

use crate::use_case::error::InternalError;

/// Interactor type which can hash password with some algorithm.
pub trait PasswordHasher {
    /// Hashes provided password with some algorithm.
    ///
    /// Returns [`String`] with hashed password.
    fn hash(&self, password: &str) -> Result<String, InternalError>;
}

/// Interactor type which can verify password by its hash.
pub trait PasswordHashVerifier {
    /// Verifies provided password with its hash.
    ///
    /// Returns `true` if the hash of provided password and
    /// provided password hash are equal, `false` otherwise.
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, InternalError>;
}
