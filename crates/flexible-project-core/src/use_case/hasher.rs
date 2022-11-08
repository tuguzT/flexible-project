//! Hashing use cases of the Flexible Project system.

/// Interactor type which can hash password with some algorithm.
pub trait PasswordHasher {
    /// The type returned when any error occurs.
    type Error;

    /// Hashes provided password with some algorithm.
    ///
    /// Returns [`String`] with hashed password, or [error](PasswordHasher::Error) if any.
    fn hash(&self, password: &str) -> Result<String, Self::Error>;
}

/// Interactor type which can verify password by its hash.
pub trait PasswordHashVerifier {
    /// The type returned when any error occurs.
    type Error;

    /// Verifies provided password with its hash.
    ///
    /// Returns `true` if the hash of provided password and
    /// provided password hash are equal, `false otherwise`,
    /// or [error](PasswordHashVerifier::Error) if any.
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, Self::Error>;
}
