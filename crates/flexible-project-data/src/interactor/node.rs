use std::convert::Infallible;

use async_trait::async_trait;
use fp_core::model::{ErasedId, Node};
use fp_core::use_case::FindNode as CoreFindNode;

/// Interactor used to find any node of the system by its identifier.
#[derive(Default)]
pub struct FindNode;

#[async_trait]
impl CoreFindNode for FindNode {
    type Error = Infallible;

    async fn find(&self, id: ErasedId) -> Result<Option<Node>, Self::Error> {
        todo!("find by id: {}", id) // todo find in each repository
    }
}
