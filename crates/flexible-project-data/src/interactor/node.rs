use std::sync::Arc;

use async_trait::async_trait;
use fp_core::model::{ErasedId, Node, UserFilters};
use fp_core::use_case::FindNode as CoreFindNode;

use crate::data_source::user::UserDataSource;
use crate::repository::user::UserRepository;
use crate::repository::Error;

/// Interactor used to find any node of the system by its identifier.
pub struct FindNode<S>
where
    S: UserDataSource,
{
    user_repository: Arc<UserRepository<S>>,
}

impl<S> FindNode<S>
where
    S: UserDataSource,
{
    /// Creates new find node interactor.
    pub fn new(user_repository: Arc<UserRepository<S>>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<S> CoreFindNode for FindNode<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = Error;

    async fn find(&self, id: ErasedId) -> Result<Option<Node>, Self::Error> {
        let filter = UserFilters {
            ids: vec![id.with_owner()],
            names: vec![],
        };
        let user = self.user_repository.read(filter).await?.first().cloned();
        let user = user.map(Node::from);
        Ok(user)
    }
}
