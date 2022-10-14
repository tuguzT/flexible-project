/// Interactor type which can generate
/// new globally unique identifier of some object.
pub trait GUIDGenerator {
    /// The type of identifier.
    type Id;

    /// Generates new globally unique identifier.
    fn generate(&self) -> Self::Id;
}
