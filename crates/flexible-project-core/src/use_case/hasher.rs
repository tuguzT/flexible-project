//! Hashing use cases of the Flexible Project system.

use std::sync::Arc;

use async_trait::async_trait;

use super::error::InternalError;

/// Interactor type which can hash password with some algorithm.
#[async_trait]
pub trait PasswordHasher: Send + Sync {
    /// Hashes provided password with some algorithm.
    ///
    /// Returns [`String`] with hashed password.
    async fn hash(&self, password: String) -> Result<String, InternalError>;
}

#[async_trait]
impl<T> PasswordHasher for &T
where
    T: PasswordHasher,
{
    async fn hash(&self, password: String) -> Result<String, InternalError> {
        (**self).hash(password).await
    }
}

#[async_trait]
impl<T> PasswordHasher for Arc<T>
where
    T: PasswordHasher,
{
    async fn hash(&self, password: String) -> Result<String, InternalError> {
        (**self).hash(password).await
    }
}

/// Interactor type which can verify password by its hash.
#[async_trait]
pub trait PasswordHashVerifier: Send + Sync {
    /// Verifies provided password with its hash.
    ///
    /// Returns `true` if the hash of provided password and
    /// provided password hash are equal, `false` otherwise.
    async fn verify(&self, password: String, password_hash: String) -> Result<bool, InternalError>;
}

#[async_trait]
impl<T> PasswordHashVerifier for &T
where
    T: PasswordHashVerifier,
{
    async fn verify(&self, password: String, password_hash: String) -> Result<bool, InternalError> {
        (**self).verify(password, password_hash).await
    }
}

#[async_trait]
impl<T> PasswordHashVerifier for Arc<T>
where
    T: PasswordHashVerifier,
{
    async fn verify(&self, password: String, password_hash: String) -> Result<bool, InternalError> {
        (**self).verify(password, password_hash).await
    }
}
