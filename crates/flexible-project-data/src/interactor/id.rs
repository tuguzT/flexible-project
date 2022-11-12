//! Identifier use case implementations of the Flexible Project system.

use fp_core::model::id::ErasedId;
use fp_core::use_case::error::InternalError;
use fp_core::use_case::id::IdGenerator as CoreIdGenerator;
use uuid::Uuid;

/// Interactor used to generate globally unique identifier.
#[derive(Debug, Clone, Default)]
pub struct IdGenerator;

impl CoreIdGenerator for IdGenerator {
    fn generate(&self) -> Result<ErasedId, InternalError> {
        let id = Uuid::new_v4().to_string().into();
        Ok(id)
    }
}
