use async_trait::async_trait;

use crate::model::Node;

/// Interactor type which can find any node of the system by its identifier.
#[async_trait]
pub trait FindNode {
    /// Node type to be retrieved.
    type Node: Node;

    /// The type returned when any error occurs.
    type Error;

    /// Returns [`Some`] if node by provided identifier exists,
    /// returns [`None`] otherwise.
    async fn find(&self, id: <Self::Node as Node>::Id) -> Result<Option<Self::Node>, Self::Error>;
}
