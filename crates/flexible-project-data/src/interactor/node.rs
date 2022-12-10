//! Node use case implementations of the Flexible Project system.

use std::sync::Arc;

use async_trait::async_trait;
use fp_core::{
    model::{
        id::{ErasedId, IdFilters},
        node::Node,
        user::UserFilters,
    },
    use_case::{error::InternalError, user::FilterUsers},
};

mod core {
    pub use fp_core::use_case::node::FindNode;
}

/// Interactor used to find any node of the system by its identifier.
pub struct FindNode {
    filter: Arc<dyn FilterUsers>,
}

impl FindNode {
    /// Creates new find node interactor.
    pub fn new(filter: Arc<dyn FilterUsers>) -> Self {
        Self { filter }
    }
}

#[async_trait]
impl core::FindNode for FindNode {
    async fn find(&self, id: ErasedId) -> Result<Option<Node>, InternalError> {
        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(id.with_owner()).build())
            .build();
        let user = self.filter.filter(filters).await?.first().cloned();
        let user = user.map(Node::from);
        Ok(user)
    }
}
