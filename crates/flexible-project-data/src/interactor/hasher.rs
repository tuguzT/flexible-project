//! Hashing use case implementations of the Flexible Project system.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier as _, Version};
use async_trait::async_trait;
use derive_more::{Display, Error, From};
use fp_core::use_case::error::InternalError;
use futures::executor::block_on;
use ouroboros::self_referencing;
use tokio::runtime::Handle;

mod core {
    pub use fp_core::use_case::hasher::{PasswordHashVerifier, PasswordHasher};
}

/// Interactor for password hashing with Argon2 algorithm.
#[self_referencing]
pub struct PasswordHasher {
    secret: Option<String>,
    #[borrows(secret)]
    #[not_covariant]
    hasher: Argon2<'this>,
}

/// Error that may occur when creating new password hasher interactor with some secret.
#[derive(Error, Debug, Display, From, Clone)]
#[display(fmt = "invalid secret provided")]
pub struct WithSecretError(argon2::Error);

impl PasswordHasher {
    /// Creates new password hasher interactor with some secret.
    pub fn new_with_secret(secret: String) -> Result<Self, WithSecretError> {
        PasswordHasherTryBuilder {
            secret: Some(secret),
            hasher_builder: |secret| {
                let secret = secret.as_ref().map(String::as_bytes).unwrap_or_default();
                let algorithm = Algorithm::default();
                let version = Version::default();
                let params = Params::default();
                let argon2 = Argon2::new_with_secret(secret, algorithm, version, params)?;
                Ok(argon2)
            },
        }
        .try_build()
    }
}

impl Default for PasswordHasher {
    /// Creates new password hasher interactor without any secret.
    fn default() -> Self {
        PasswordHasherBuilder {
            secret: None,
            hasher_builder: |_| Argon2::default(),
        }
        .build()
    }
}

#[async_trait]
impl core::PasswordHasher for PasswordHasher {
    async fn hash(&self, password: String) -> Result<String, InternalError> {
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .with_hasher(|hasher| {
                let handle = Handle::current();
                let _guard = handle.enter();
                block_on(async { hasher.hash_password(password, &salt) })
            })
            .map_err(InternalError::new)?;
        Ok(password_hash.to_string())
    }
}

#[async_trait]
impl core::PasswordHashVerifier for PasswordHasher {
    async fn verify(&self, password: String, password_hash: String) -> Result<bool, InternalError> {
        let password = password.as_bytes();
        let password_hash = password_hash
            .as_str()
            .try_into()
            .map_err(InternalError::new)?;
        let result = self.with_hasher(|hasher| {
            let handle = Handle::current();
            let _guard = handle.enter();
            block_on(async { hasher.verify_password(password, &password_hash) })
        });
        match result {
            Ok(_) => Ok(true),
            Err(error) => match error {
                argon2::password_hash::Error::Password => Ok(false),
                error => Err(InternalError::new(error)),
            },
        }
    }
}
