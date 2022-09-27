use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::repository::Repository;

/// Trait for repository which can retrieve all the data
/// of type [`Item`](Repository::Item).
#[async_trait]
pub trait ReadAll: Repository {
    /// The type returned when any error occurs.
    type Error;

    /// Returns all the data of type [`Item`](Repository::Item).
    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error>;
}

/// Trait for repository which can retrieve the item by its identifier.
#[async_trait]
pub trait ReadById: Repository
where
    Self::Item: Identifiable,
{
    /// The type returned when any error occurs.
    type Error;

    /// Returns [`Some`] with an item found by its identifier
    /// or [`None`] if there is no item by provided identifier.
    async fn read_by_id(
        &self,
        id: <Self::Item as Identifiable>::Id,
    ) -> Result<Option<Self::Item>, Self::Error>;
}
