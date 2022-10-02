use async_trait::async_trait;
use fp_core::model::Node;

use crate::repository::Repository;

/// Repository type which can delete an item by its value.
#[async_trait]
pub trait Delete: Repository {
    /// Deletes provided item.
    ///
    /// Item will be deleted only if it is equal
    /// to the item stored in the repository.
    async fn delete(&self, item: Self::Item) -> Result<Option<Self::Item>, Self::Error>;
}

/// Repository type which can delete an item by its identifier.
#[async_trait]
pub trait DeleteById: Repository
where
    Self::Item: Node,
{
    /// Deletes an item by provided identifier.
    async fn delete_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error>;
}
