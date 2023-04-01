use auto_impl::auto_impl;

use crate::model::UserId;

/// Generator of unique user identifiers.
#[auto_impl(&, Box, Rc, Arc)]
pub trait IdGenerator {
    /// Type of error which is returned when a repository fails to generate new identifier.
    type Error;

    /// Generates unique user identifier.
    fn generate_id(&self) -> Result<UserId, Self::Error>;
}
