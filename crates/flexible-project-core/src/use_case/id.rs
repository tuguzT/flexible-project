use crate::model::id::ErasedId;

/// Interactor type which can generate new globally unique identifier.
pub trait IdGenerator {
    /// The type returned when any error occurs.
    type Error;

    /// Generates new globally unique identifier.
    fn generate(&self) -> Result<ErasedId, Self::Error>;
}
