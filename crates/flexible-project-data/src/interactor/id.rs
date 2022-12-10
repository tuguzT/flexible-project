//! Identifier use case implementations of the Flexible Project system.

use async_trait::async_trait;
use fp_core::model::id::ErasedId;
use fp_core::use_case::error::InternalError;
use tokio::task::spawn_blocking;
use uuid::Uuid;

mod core {
    pub use fp_core::use_case::id::IdGenerator;
}

/// Interactor used to generate globally unique identifier.
#[derive(Debug, Clone, Default)]
pub struct IdGenerator(());

#[async_trait]
impl core::IdGenerator for IdGenerator {
    async fn generate(&self) -> Result<ErasedId, InternalError> {
        let future = spawn_blocking(|| Uuid::new_v4().to_string().into());
        let id = future.await.map_err(InternalError::new)?;
        Ok(id)
    }
}
