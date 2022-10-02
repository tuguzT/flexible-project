use std::iter::FromIterator;

use async_trait::async_trait;
use fp_core::model::Node;
use tokio::sync::RwLock;

use crate::data_source::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::data_source::user::UserDataSource;
use crate::data_source::DataSource;
use crate::model::User;
use crate::Error;

/// Mock user data source which stores users inside of [`Vec`].
#[derive(Default)]
pub struct MockUserDataSource(RwLock<Vec<User>>);

impl DataSource for MockUserDataSource {
    type Item = User;
    type Error = Error;
}

#[async_trait]
impl Clear for MockUserDataSource {
    async fn clear(&self) -> Result<(), Self::Error> {
        let mut vec = self.0.write().await;
        vec.clear();
        Ok(())
    }
}

#[async_trait]
impl Delete for MockUserDataSource {
    async fn delete(&self, item: Self::Item) -> Result<Option<Self::Item>, Self::Error> {
        let mut vec = self.0.write().await;
        let index = vec.iter().position(|x| x == &item);
        let user = index.map(|index| vec.swap_remove(index));
        Ok(user)
    }
}

#[async_trait]
impl DeleteById for MockUserDataSource {
    async fn delete_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let mut vec = self.0.write().await;
        let index = vec.iter().position(|x| x.id() == id);
        let user = index.map(|index| vec.swap_remove(index));
        Ok(user)
    }
}

#[async_trait]
impl ReadAll for MockUserDataSource {
    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let vec = self.0.read().await;
        Ok(vec.clone())
    }
}

#[async_trait]
impl ReadById for MockUserDataSource {
    async fn read_by_id(
        &self,
        id: <Self::Item as Node>::Id,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let vec = self.0.read().await;
        Ok(vec.iter().find(|x| x.id() == id).cloned())
    }
}

#[async_trait]
impl Save for MockUserDataSource {
    async fn save(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let mut vec = self.0.write().await;
        let by_id = vec.iter().position(|x| x.id() == item.id);
        match by_id {
            Some(idx) => vec[idx] = item.clone(),
            None => vec.push(item.clone()),
        }
        Ok(item)
    }
}

impl UserDataSource for MockUserDataSource {}

impl FromIterator<User> for MockUserDataSource {
    fn from_iter<T: IntoIterator<Item = User>>(iter: T) -> Self {
        let vec = FromIterator::from_iter(iter);
        Self(RwLock::new(vec))
    }
}
