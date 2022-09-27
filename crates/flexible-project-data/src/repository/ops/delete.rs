use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::repository::Repository;

/// Trait for repository which can delete an item by its value.
#[async_trait]
pub trait Delete: Repository {
    /// The type returned when any error occurs.
    type Error;

    /// Deletes provided item.
    ///
    /// Item will be deleted only if it is equal
    /// to the item stored in the repository.
    async fn delete(&self, item: Self::Item) -> Result<Self::Item, Self::Error>;
}

/// Trait for repository which can delete an item by its identifier.
#[async_trait]
pub trait DeleteById: Repository
where
    Self::Item: Identifiable,
{
    /// The type returned when any error occurs.
    type Error;

    /// Deletes an item by provided identifier.
    async fn delete_by_id(
        &self,
        id: <Self::Item as Identifiable>::Id,
    ) -> Result<Self::Item, Self::Error>;
}
