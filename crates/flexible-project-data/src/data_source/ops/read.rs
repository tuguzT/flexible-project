use async_trait::async_trait;
use fp_core::model::Node;

use crate::data_source::DataSource;

/// Data source type which can retrieve all the data
/// of stored [type](DataSource::Item) from the storage.
#[async_trait]
pub trait ReadAll: DataSource {
    /// Returns all the data of stored [type](DataSource::Item) from the storage.
    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

/// Trait for data source which can retrieve the item
/// by its identifier from the storage.
#[async_trait]
pub trait ReadById: DataSource
where
    Self::Item: Node,
{
    /// Returns [`Some`] with an item found by its identifier in the storage
    /// or [`None`] if there is no item by provided identifier.
    async fn read_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error>;
}
