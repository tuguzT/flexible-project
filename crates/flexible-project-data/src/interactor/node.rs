use std::convert::Infallible;

use async_trait::async_trait;
use fp_core::model::Node as CoreNode;
use fp_core::use_case::FindNode as CoreFindNode;

use crate::model::Node;

/// Interactor used to find any node of the system by its identifier.
pub struct FindNode;

#[async_trait]
impl CoreFindNode for FindNode {
    type Node = Node;

    type Error = Infallible;

    async fn find(
        &self,
        id: <Self::Node as CoreNode>::Id,
    ) -> Result<Option<Self::Node>, Self::Error> {
        todo!("find by id: {}", id)
    }
}
