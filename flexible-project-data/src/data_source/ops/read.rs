use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::data_source::DataSource;

/// Trait for data source which can retrieve all the data
/// of type [`Item`](DataSource::Item) from the storage.
#[async_trait]
pub trait ReadAll: DataSource {
    /// Returns all the data of type [`Item`](DataSource::Item) from the storage.
    async fn read_all(&self) -> Vec<Self::Item>;
}

/// Trait for data source which can retrieve the item
/// by its identifier from the storage.
#[async_trait]
pub trait ReadById: DataSource {
    /// Returns [`Some`] with an item found by its identifier in the storage
    /// or [`None`] if there is no item by provided identifier.
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item>;
}
