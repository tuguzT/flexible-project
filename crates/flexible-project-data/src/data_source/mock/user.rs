use std::iter::FromIterator;

use async_trait::async_trait;
use fp_core::model::Identifiable;
use tokio::sync::RwLock;

use crate::data_source::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::data_source::user::UserDataSource;
use crate::data_source::DataSource;
use crate::model::User;

/// Mock user data source which stores users inside of [`Vec`].
#[derive(Default)]
pub struct MockUserDataSource(RwLock<Vec<User>>);

impl DataSource for MockUserDataSource {
    type Item = User;
}

#[async_trait]
impl Clear for MockUserDataSource {
    async fn clear(&self) {
        let mut vec = self.0.write().await;
        vec.clear()
    }
}

#[async_trait]
impl Delete for MockUserDataSource {
    async fn delete(&self, item: Self::Item) -> Option<Self::Item> {
        let mut vec = self.0.write().await;
        let index = vec.iter().position(|x| x == &item)?;
        let user = vec.swap_remove(index);
        Some(user)
    }
}

#[async_trait]
impl DeleteById for MockUserDataSource {
    async fn delete_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        let mut vec = self.0.write().await;
        let index = vec.iter().position(|x| x.id() == id)?;
        let user = vec.swap_remove(index);
        Some(user)
    }
}

#[async_trait]
impl ReadAll for MockUserDataSource {
    async fn read_all(&self) -> Vec<Self::Item> {
        let vec = self.0.read().await;
        vec.clone()
    }
}

#[async_trait]
impl ReadById for MockUserDataSource {
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        let vec = self.0.read().await;
        vec.iter().find(|x| x.id() == id).cloned()
    }
}

#[async_trait]
impl Save for MockUserDataSource {
    async fn save(&self, item: Self::Item) -> Self::Item {
        let mut vec = self.0.write().await;
        let by_id = vec.iter().position(|x| x.id() == item.id);
        match by_id {
            Some(idx) => vec[idx] = item.clone(),
            None => vec.push(item.clone()),
        }
        item
    }
}

impl UserDataSource for MockUserDataSource {}

impl FromIterator<User> for MockUserDataSource {
    fn from_iter<T: IntoIterator<Item = User>>(iter: T) -> Self {
        let vec = FromIterator::from_iter(iter);
        Self(RwLock::new(vec))
    }
}
