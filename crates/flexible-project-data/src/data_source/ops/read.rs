use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::data_source::DataSource;

/// Trait for data source which can retrieve all the data
/// of type [`Item`](DataSource::Item) from the storage.
#[async_trait]
pub trait ReadAll: DataSource {
    /// The type returned when any error occurs.
    type Error;

    /// Returns all the data of type [`Item`](DataSource::Item) from the storage.
    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

/// Trait for data source which can retrieve the item
/// by its identifier from the storage.
#[async_trait]
pub trait ReadById: DataSource
where
    Self::Item: Identifiable,
{
    /// The type returned when any error occurs.
    type Error;

    /// Returns [`Some`] with an item found by its identifier in the storage
    /// or [`None`] if there is no item by provided identifier.
    async fn read_by_id(
        &self,
        id: <Self::Item as Identifiable>::Id,
    ) -> Result<Option<Self::Item>, Self::Error>;
}
