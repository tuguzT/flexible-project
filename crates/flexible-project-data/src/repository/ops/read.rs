use async_trait::async_trait;
use fp_core::model::Node;

use crate::repository::Repository;

/// Repository type which can retrieve all the data
/// of stored [type](Repository::Item).
#[async_trait]
pub trait ReadAll: Repository {
    /// Returns all the data of stored [type](Repository::Item).
    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

/// Repository type which can retrieve the item by its identifier.
#[async_trait]
pub trait ReadById: Repository
where
    Self::Item: Node,
{
    /// Returns [`Some`] with an item found by its identifier
    /// or [`None`] if there is no item by provided identifier.
    async fn read_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error>;
}
