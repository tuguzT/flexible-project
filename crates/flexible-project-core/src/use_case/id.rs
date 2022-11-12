//! Identifier use cases of the Flexible Project system.

use crate::model::id::ErasedId;
use crate::use_case::error::InternalError;

/// Interactor type which can generate new globally unique identifier.
pub trait IdGenerator {
    /// Generates new globally unique identifier.
    fn generate(&self) -> Result<ErasedId, InternalError>;
}
