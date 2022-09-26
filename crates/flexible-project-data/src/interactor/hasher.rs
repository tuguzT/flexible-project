use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher as _, PasswordVerifier as _};
use fp_core::use_case::PasswordHasher as CorePasswordHasher;

/// Interactor for password hashing with Argon2 algorithm.
pub struct PasswordHasher<'key> {
    hasher: Argon2<'key>,
}

impl<'key> PasswordHasher<'key> {
    /// Creates new password hasher interactor.
    pub fn new(hasher: Argon2<'key>) -> Self {
        Self { hasher }
    }
}

impl CorePasswordHasher for PasswordHasher<'_> {
    fn hash(&self, password: &str) -> String {
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self.hasher.hash_password(password, &salt).unwrap();
        password_hash.to_string()
    }

    fn verify(&self, password: &str, password_hash: &str) -> bool {
        let password = password.as_bytes();
        let password_hash = password_hash.try_into().unwrap();
        self.hasher
            .verify_password(password, &password_hash)
            .is_ok()
    }
}
