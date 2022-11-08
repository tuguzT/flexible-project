//! Hashing use case implementations of the Flexible Project system.

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier as _, Version};
use derive_more::{Display, Error, From};
use fp_core::use_case::hasher::{PasswordHashVerifier, PasswordHasher as CorePasswordHasher};
use ouroboros::self_referencing;

/// Interactor for password hashing with Argon2 algorithm.
#[self_referencing]
pub struct PasswordHasher {
    secret: Option<String>,
    #[borrows(secret)]
    #[covariant]
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

/// Error that may occur when password is being hashed by some algorithm.
#[derive(Error, Debug, Display, From, Clone)]
#[display(fmt = "password hashing failed")]
pub struct PasswordHashError(argon2::password_hash::Error);

impl CorePasswordHasher for PasswordHasher {
    type Error = PasswordHashError;

    fn hash(&self, password: &str) -> Result<String, Self::Error> {
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.borrow_hasher().hash_password(password, &salt)?;
        Ok(password_hash.to_string())
    }
}

/// Error that may occur when verifying password with its hash.
#[derive(Error, Debug, Display, From, Clone)]
#[display(fmt = "password hash verification failed")]
pub struct PasswordHashVerifyError(argon2::password_hash::Error);

impl PasswordHashVerifier for PasswordHasher {
    type Error = PasswordHashVerifyError;

    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, Self::Error> {
        let password = password.as_bytes();
        let password_hash = password_hash.try_into()?;
        let result = self
            .borrow_hasher()
            .verify_password(password, &password_hash);
        match result {
            Ok(_) => Ok(true),
            Err(error) => match error {
                argon2::password_hash::Error::Password => Ok(false),
                error => Err(error.into()),
            },
        }
    }
}
