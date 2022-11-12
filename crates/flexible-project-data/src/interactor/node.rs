//! Node use case implementations of the Flexible Project system.

use async_trait::async_trait;
use fp_core::model::id::{ErasedId, IdFilters};
use fp_core::model::node::Node;
use fp_core::model::user::UserFilters;
use fp_core::use_case::error::InternalError;
use fp_core::use_case::node::FindNode as CoreFindNode;
use fp_core::use_case::user::FilterUsers as _;

use crate::data_source::user::UserDataSource;
use crate::interactor::user::FilterUsers;

/// Interactor used to find any node of the system by its identifier.
pub struct FindNode<S>
where
    S: UserDataSource,
{
    filter: FilterUsers<S>,
}

impl<S> FindNode<S>
where
    S: UserDataSource,
{
    /// Creates new find node interactor.
    pub fn new(filter: FilterUsers<S>) -> Self {
        Self { filter }
    }
}

#[async_trait]
impl<S> CoreFindNode for FindNode<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn find(&self, id: ErasedId) -> Result<Option<Node>, InternalError> {
        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(id.with_owner()).build())
            .build();
        let user = self.filter.filter(filters).await?.first().cloned();
        let user = user.map(Node::from);
        Ok(user)
    }
}
