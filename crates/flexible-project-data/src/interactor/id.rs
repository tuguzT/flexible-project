use fp_core::use_case::GUIDGenerator as CoreGUIDGenerator;
use uuid::Uuid;

/// Interactor used to generate globally unique identifier.
#[derive(Debug, Clone, Default)]
pub struct GUIDGenerator;

impl CoreGUIDGenerator for GUIDGenerator {
    type Id = Uuid;

    fn generate(&self) -> Self::Id {
        Uuid::new_v4()
    }
}
