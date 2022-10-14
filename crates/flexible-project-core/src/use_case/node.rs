use async_trait::async_trait;

use crate::model::{ErasedId, Node};

/// Interactor type which can find any node of the system by its identifier.
#[async_trait]
pub trait FindNode {
    /// The type returned when any error occurs.
    type Error;

    /// Returns [`Some`] if node by provided identifier exists, [`None`] otherwise.
    async fn find(&self, id: ErasedId) -> Result<Option<Node>, Self::Error>;
}
