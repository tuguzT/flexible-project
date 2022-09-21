use std::iter::FromIterator;

use async_trait::async_trait;
use fp_core::model::Identifiable;

use crate::data_source::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};
use crate::data_source::user::UserDataSource;
use crate::data_source::DataSource;
use crate::model::User;

/// Mock user data source which stores users inside of [`Vec`].
#[derive(Default)]
pub struct MockUserDataSource(Vec<User>);

impl DataSource for MockUserDataSource {
    type Item = User;
}

#[async_trait]
impl Clear for MockUserDataSource {
    async fn clear(&mut self) {
        self.0.clear()
    }
}

#[async_trait]
impl Delete for MockUserDataSource {
    async fn delete(&mut self, item: Self::Item) -> Option<Self::Item> {
        let index = self.0.iter().position(|x| x == &item)?;
        let user = self.0.swap_remove(index);
        Some(user)
    }
}

#[async_trait]
impl DeleteById for MockUserDataSource {
    async fn delete_by_id(&mut self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        let index = self.0.iter().position(|x| x.id() == id)?;
        let user = self.0.swap_remove(index);
        Some(user)
    }
}

#[async_trait]
impl ReadAll for MockUserDataSource {
    async fn read_all(&self) -> Vec<Self::Item> {
        self.0.clone()
    }
}

#[async_trait]
impl ReadById for MockUserDataSource {
    async fn read_by_id(&self, id: <Self::Item as Identifiable>::Id) -> Option<Self::Item> {
        self.0.iter().find(|x| x.id() == id).cloned()
    }
}

#[async_trait]
impl Save for MockUserDataSource {
    async fn save(&mut self, item: Self::Item) -> Self::Item {
        let by_id = self.0.iter().position(|x| x.id() == item.id);
        match by_id {
            Some(idx) => self.0[idx] = item.clone(),
            None => self.0.push(item.clone()),
        }
        item
    }
}

impl UserDataSource for MockUserDataSource {}

impl FromIterator<User> for MockUserDataSource {
    fn from_iter<T: IntoIterator<Item = User>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter))
    }
}
