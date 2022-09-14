//! Repositories for users of the Flexible Project system.

use async_trait::async_trait;
use fp_core::model::{Identifiable, User};

use crate::data_source::user::UserDataSource;
use crate::repository::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::repository::Repository;

/// CRUD repository for users of the Flexible Project system.
pub struct UserRepository<S>
where
    S: UserDataSource,
    S::Item: User,
{
    data_source: S,
}

impl<S> UserRepository<S>
where
    S: UserDataSource,
    S::Item: User,
{
    /// Creates new repository from provided data source.
    pub fn new(data_source: S) -> Self {
        Self { data_source }
    }
}

impl<S> Repository for UserRepository<S>
where
    S: UserDataSource,
    S::Item: User,
{
    type Item = S::Item;
}

#[async_trait]
impl<S> Clear for UserRepository<S>
where
    S: UserDataSource + Send,
    S::Item: User,
{
    async fn clear(&mut self) {
        self.data_source.clear().await
    }
}

#[async_trait]
impl<S> Delete for UserRepository<S>
where
    S: UserDataSource + Send,
    S::Item: User + Send,
{
    async fn delete(&mut self, item: Self::Item) {
        self.data_source.delete(item).await
    }
}

#[async_trait]
impl<S> DeleteById for UserRepository<S>
where
    S: UserDataSource + Send,
    S::Item: User,
    <S::Item as Identifiable>::Id: Send,
{
    async fn delete_by_id(&mut self, id: <Self::Item as Identifiable>::Id) {
        self.data_source.delete_by_id(id).await
    }
}

#[async_trait]
impl<S> ReadAll for UserRepository<S>
where
    S: UserDataSource + Send + Sync,
    S::Item: User,
{
    async fn read_all(&self) -> Vec<Self::Item> {
        self.data_source.read_all().await
    }
}

#[async_trait]
impl<S> ReadById for UserRepository<S>
where
    S: UserDataSource + Send + Sync,
    S::Item: User,
    <S::Item as Identifiable>::Id: Send,
{
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        self.data_source.read_by_id(id).await
    }
}

#[async_trait]
impl<S> Save for UserRepository<S>
where
    S: UserDataSource + Send,
    S::Item: User + Send,
{
    async fn save(&mut self, item: Self::Item) -> Self::Item {
        self.data_source.save(item).await
    }
}
