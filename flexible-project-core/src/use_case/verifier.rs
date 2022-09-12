/// Trait for objects which verify username provided by user.
pub trait UsernameVerifier {
    /// Verifies username provided by user.
    ///
    /// Returns `true` if provided username is valid, `false` otherwise.
    fn verify(&self, username: &str) -> bool;
}

/// Trait for objects which verify password provided by user.
pub trait PasswordVerifier {
    /// Verifies password provided by user.
    ///
    /// Returns `true` if provided password is valid, `false` otherwise.
    fn verify(&self, password: &str) -> bool;
}
