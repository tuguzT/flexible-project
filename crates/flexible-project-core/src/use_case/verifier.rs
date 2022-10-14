use crate::model::UserCredentials;

/// Interactor type which can verify username provided by user.
pub trait UsernameVerifier {
    /// The type returned when any error occurs.
    type Error;

    /// Verifies username provided by user.
    ///
    /// Returns `true` if provided username is valid, `false` otherwise.
    fn verify(&self, username: &str) -> Result<bool, Self::Error>;
}

/// Interactor type which can verify password provided by user.
pub trait PasswordVerifier {
    /// The type returned when any error occurs.
    type Error;

    /// Verifies password provided by user.
    ///
    /// Returns `true` if provided password is valid, `false` otherwise.
    fn verify(&self, password: &str) -> Result<bool, Self::Error>;
}

/// Interactor type which can verify credentials provided by user.
pub trait UserCredentialsVerifier {
    /// The type returned when any error occurs.
    type Error;

    /// Verifies credentials provided by user.
    ///
    /// Returns `true` if provided credentials are valid, `false` otherwise.
    fn verify(&self, credentials: &UserCredentials) -> Result<bool, Self::Error>;
}
