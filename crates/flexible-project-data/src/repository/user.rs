//! Repositories for users of the Flexible Project system.

use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::data_source::user::UserDataSource;
use crate::repository::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::repository::Repository;

/// User repository of the Flexible Project system.
pub struct UserRepository<S>(S)
where
    S: UserDataSource;

impl<S> UserRepository<S>
where
    S: UserDataSource,
{
    /// Creates new repository from provided data source.
    pub fn new(data_source: S) -> Self {
        Self(data_source)
    }
}

impl<S> Repository for UserRepository<S>
where
    S: UserDataSource,
{
    type Item = S::Item;
}

#[async_trait]
impl<S> Clear for UserRepository<S>
where
    S: UserDataSource + Send,
{
    async fn clear(&mut self) {
        self.0.clear().await
    }
}

#[async_trait]
impl<S> Delete for UserRepository<S>
where
    S: UserDataSource + Send,
{
    async fn delete(&mut self, item: Self::Item) {
        self.0.delete(item).await
    }
}

#[async_trait]
impl<S> DeleteById for UserRepository<S>
where
    S: UserDataSource + Send,
{
    async fn delete_by_id(&mut self, id: <Self::Item as Identifiable>::Id) {
        self.0.delete_by_id(id).await
    }
}

#[async_trait]
impl<S> ReadAll for UserRepository<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn read_all(&self) -> Vec<Self::Item> {
        self.0.read_all().await
    }
}

#[async_trait]
impl<S> ReadById for UserRepository<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        self.0.read_by_id(id).await
    }
}

#[async_trait]
impl<S> Save for UserRepository<S>
where
    S: UserDataSource + Send,
{
    async fn save(&mut self, item: Self::Item) -> Self::Item {
        self.0.save(item).await
    }
}
