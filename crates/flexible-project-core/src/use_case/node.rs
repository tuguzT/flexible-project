//! Node use cases of the Flexible Project system.

use async_trait::async_trait;

use crate::model::id::ErasedId;
use crate::model::node::Node;
use crate::use_case::error::InternalError;

/// Interactor type which can find any node of the system by its identifier.
#[async_trait]
pub trait FindNode: Send + Sync + 'static {
    /// Returns [`Some`] if node by provided identifier exists, [`None`] otherwise.
    async fn find(&self, id: ErasedId) -> Result<Option<Node>, InternalError>;
}
