//! Identifier use cases of the Flexible Project system.

use async_trait::async_trait;

use crate::model::id::ErasedId;

use super::error::InternalError;

/// Interactor type which can generate new globally unique identifier.
#[async_trait]
pub trait IdGenerator: Send + Sync {
    /// Generates new globally unique identifier.
    async fn generate(&self) -> Result<ErasedId, InternalError>;
}
