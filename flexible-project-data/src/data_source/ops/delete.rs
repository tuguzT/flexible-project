use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::data_source::DataSource;

/// Trait for data source which can delete an item by its value.
#[async_trait]
pub trait Delete: DataSource {
    /// Deletes provided item from the storage.
    ///
    /// Item will be deleted only if it is equal
    /// to the item stored in the storage.
    async fn delete(&self, item: Self::Item);
}

/// Trait for data source which can delete an item by its identifier.
#[async_trait]
pub trait DeleteById: DataSource {
    /// Deletes item from the storage by provided identifier.
    async fn delete_by_id(&self, id: <Self::Item as Identifiable>::Id);
}
