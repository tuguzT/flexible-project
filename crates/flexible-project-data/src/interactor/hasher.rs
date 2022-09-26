use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier, Result, Version};
use fp_core::use_case::PasswordHasher as CorePasswordHasher;
use ouroboros::self_referencing;

/// Interactor for password hashing with Argon2 algorithm.
#[self_referencing]
pub struct PasswordHasher {
    secret: Option<String>,
    #[borrows(secret)]
    #[covariant]
    hasher: Argon2<'this>,
}

impl PasswordHasher {
    /// Creates new password hasher interactor with some secret.
    pub fn new_with_secret(secret: String) -> Result<Self> {
        PasswordHasherTryBuilder {
            secret: Some(secret),
            hasher_builder: |secret| {
                let secret = secret.as_ref().map(String::as_bytes).unwrap_or_default();
                let algorithm = Algorithm::default();
                let version = Version::default();
                let params = Params::default();
                Argon2::new_with_secret(secret, algorithm, version, params)
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

impl CorePasswordHasher for PasswordHasher {
    fn hash(&self, password: &str) -> String {
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.borrow_hasher().hash_password(password, &salt).unwrap();
        password_hash.to_string()
    }

    fn verify(&self, password: &str, password_hash: &str) -> bool {
        let password = password.as_bytes();
        let password_hash = password_hash.try_into().unwrap();
        self.borrow_hasher()
            .verify_password(password, &password_hash)
            .is_ok()
    }
}
