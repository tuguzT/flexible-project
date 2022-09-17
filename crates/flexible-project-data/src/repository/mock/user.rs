use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::data_source::mock::MockUserDataSource;
use crate::data_source::ops::{
    Clear as _, Delete as _, DeleteById as _, ReadAll as _, ReadById as _, Save as _,
};
use crate::model::UserData;
use crate::repository::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::repository::user::UserRepository;
use crate::repository::Repository;

/// Mock implementation of user repository of the Flexible Project system.
#[derive(Default)]
pub struct MockUserRepository(MockUserDataSource);

impl MockUserRepository {
    /// Creates new mock user repository from mock user data source.
    pub fn new(data_source: MockUserDataSource) -> Self {
        Self(data_source)
    }
}

impl Repository for MockUserRepository {
    type Item = UserData;
}

#[async_trait]
impl Clear for MockUserRepository {
    async fn clear(&mut self) {
        self.0.clear().await
    }
}

#[async_trait]
impl Delete for MockUserRepository {
    async fn delete(&mut self, item: Self::Item) {
        self.0.delete(item).await
    }
}

#[async_trait]
impl DeleteById for MockUserRepository {
    async fn delete_by_id(&mut self, id: <Self::Item as Identifiable>::Id) {
        self.0.delete_by_id(id).await
    }
}

#[async_trait]
impl ReadAll for MockUserRepository {
    async fn read_all(&self) -> Vec<Self::Item> {
        self.0.read_all().await
    }
}

#[async_trait]
impl ReadById for MockUserRepository {
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        self.0.read_by_id(id).await
    }
}

#[async_trait]
impl Save for MockUserRepository {
    async fn save(&mut self, item: Self::Item) -> Self::Item {
        self.0.save(item).await
    }
}

impl UserRepository for MockUserRepository {}
