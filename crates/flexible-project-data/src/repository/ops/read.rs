use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::repository::Repository;

/// Trait for repository which can retrieve all the data
/// of type [`Item`](Repository::Item).
#[async_trait]
pub trait ReadAll: Repository {
    /// Returns all the data of type [`Item`](Repository::Item).
    async fn read_all(&self) -> Vec<Self::Item>;
}

/// Trait for repository which can retrieve the item by its identifier.
#[async_trait]
pub trait ReadById: Repository {
    /// Returns [`Some`] with an item found by its identifier
    /// or [`None`] if there is no item by provided identifier.
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item>;
}
