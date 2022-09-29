use std::convert::Infallible;
use std::iter::FromIterator;

use async_trait::async_trait;
use derive_more::{Display, Error};
use fp_core::model::Node;
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
    type Error = Infallible;

    async fn clear(&self) -> Result<(), Self::Error> {
        let mut vec = self.0.write().await;
        vec.clear();
        Ok(())
    }
}

/// Error that can occur when deleting some user from the system.
#[derive(Error, Debug, Display)]
pub enum DeleteUserError {
    /// User was not found.
    #[display(fmt = "user not found")]
    NotFound,
}

#[async_trait]
impl Delete for MockUserDataSource {
    type Error = DeleteUserError;

    async fn delete(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        let mut vec = self.0.write().await;
        let index = vec
            .iter()
            .position(|x| x == &item)
            .ok_or(DeleteUserError::NotFound)?;
        let user = vec.swap_remove(index);
        Ok(user)
    }
}

#[async_trait]
impl DeleteById for MockUserDataSource {
    type Error = DeleteUserError;

    async fn delete_by_id(&self, id: <Self::Item as Node>::Id) -> Result<Self::Item, Self::Error> {
        let mut vec = self.0.write().await;
        let index = vec
            .iter()
            .position(|x| x.id() == id)
            .ok_or(DeleteUserError::NotFound)?;
        let user = vec.swap_remove(index);
        Ok(user)
    }
}

#[async_trait]
impl ReadAll for MockUserDataSource {
    type Error = Infallible;

    async fn read_all(&self) -> Result<Vec<Self::Item>, Self::Error> {
        let vec = self.0.read().await;
        Ok(vec.clone())
    }
}

#[async_trait]
impl ReadById for MockUserDataSource {
    type Error = Infallible;

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
    type Error = Infallible;

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
