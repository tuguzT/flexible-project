//! Identifier use case implementations of the Flexible Project system.

use std::convert::Infallible;

use fp_core::model::id::ErasedId;
use fp_core::use_case::id::IdGenerator as CoreIdGenerator;
use uuid::Uuid;

/// Interactor used to generate globally unique identifier.
#[derive(Debug, Clone, Default)]
pub struct IdGenerator;

impl CoreIdGenerator for IdGenerator {
    type Error = Infallible;

    fn generate(&self) -> Result<ErasedId, Self::Error> {
        let id = Uuid::new_v4().to_string().into();
        Ok(id)
    }
}
