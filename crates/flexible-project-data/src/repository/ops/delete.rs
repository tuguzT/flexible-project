use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::repository::Repository;

/// Trait for repository which can delete an item by its value.
#[async_trait]
pub trait Delete: Repository {
    /// Deletes provided item.
    ///
    /// Item will be deleted only if it is equal
    /// to the item stored in the repository.
    async fn delete(&mut self, item: Self::Item);
}

/// Trait for repository which can delete an item by its identifier.
#[async_trait]
pub trait DeleteById: Repository {
    /// Deletes an item by provided identifier.
    async fn delete_by_id(&mut self, id: <Self::Item as Identifiable>::Id);
}
