use crate::model::UserCredentials;

/// Trait for interactors which verify username provided by user.
pub trait UsernameVerifier {
    /// Verifies username provided by user.
    ///
    /// Returns `true` if provided username is valid, `false` otherwise.
    fn verify(&self, username: &str) -> bool;
}

/// Trait for interactors which verify password provided by user.
pub trait PasswordVerifier {
    /// Verifies password provided by user.
    ///
    /// Returns `true` if provided password is valid, `false` otherwise.
    fn verify(&self, password: &str) -> bool;
}

/// Trait for interactors which verify credentials provided by user.
pub trait UserCredentialsVerifier {
    /// Verifies credentials provided by user.
    ///
    /// Returns `true` if provided credentials are valid, `false` otherwise.
    fn verify<C>(&self, credentials: &C) -> bool
    where
        C: UserCredentials;
}
