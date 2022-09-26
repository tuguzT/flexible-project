/// Trait for objects which can hash password
/// with some algorithm and verify hashed password later.
pub trait PasswordHasher {
    /// Hashes provided password with some algorithm.
    ///
    /// Returns string with hashed password.
    fn hash(&self, password: &str) -> String;

    /// Verifies provided password with its hash.
    fn verify(&self, password: &str, password_hash: &str) -> bool;
}
