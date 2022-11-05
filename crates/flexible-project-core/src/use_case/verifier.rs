use crate::model::user::{UserCredentials, UserToken, UserTokenClaims};

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

/// State of [user credentials](UserCredentials) after its checking by
/// [user credentials verifier](UserCredentialsVerifier).
pub enum UserCredentialsState {
    /// [User credentials](UserCredentials) are totally valid.
    Valid,
    /// [User credentials](UserCredentials) name is invalid.
    InvalidUsername,
    /// [User credentials](UserCredentials) password is invalid.
    InvalidPassword,
}

/// Interactor type which can verify credentials provided by user.
pub trait UserCredentialsVerifier {
    /// The type returned when any error occurs.
    type Error;

    /// Verifies credentials provided by user.
    fn verify(&self, credentials: &UserCredentials) -> Result<UserCredentialsState, Self::Error>;
}

/// Interactor type which can verify user token provided by client.
pub trait UserTokenVerifier {
    /// The type returned when any error occurs.
    type Error;

    /// Verifies user token provided by client.
    ///
    /// Returns `true` if provided token is valid, `false` otherwise.
    fn verify(&self, token: &UserToken) -> Result<UserTokenClaims, Self::Error>;
}
