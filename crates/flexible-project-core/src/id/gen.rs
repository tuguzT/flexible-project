use auto_impl::auto_impl;

use super::{ErasedOwner, Id};

/// Generator of globally unique identifiers with provided owner.
#[auto_impl(&, &mut, Box, Rc, Arc)]
pub trait GenerateId<Owner = ErasedOwner> {
    /// Type of error which is returned when a repository fails to generate new identifier.
    type Error;

    /// Generates globally unique identifier with provided owner.
    fn generate_id(&self) -> Result<Id<Owner>, Self::Error>;
}
