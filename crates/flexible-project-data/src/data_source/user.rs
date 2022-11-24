//! Data sources for users of the Flexible Project system.

use std::sync::Arc;

use async_trait::async_trait;
use fp_core::model::id::Id;
use fp_core::model::user::{User, UserFilters};

use crate::data_source::{DataSource, Result};

/// User data source type of the Flexible Project system.
#[async_trait]
pub trait UserDataSource: DataSource<Item = User> {
    /// Create new user from [user data](User) and password hash
    /// which will be saved for this user.
    async fn create(&self, user: Self::Item, password_hash: String) -> Result<Self::Item>;

    /// Find users by provided [filters](UserFilters).
    async fn read(&self, filter: UserFilters) -> Result<Vec<Self::Item>>;

    /// Update user which has the same [identifier](fp_core::model::id::Id)
    /// from the user parameter with provided [user data](User).
    async fn update(&self, user: Self::Item) -> Result<Option<Self::Item>>;

    /// Delete user with the same data as in the user parameter.
    async fn delete(&self, user: Self::Item) -> Result<Option<Self::Item>>;

    /// Retrieve password hash from the user by its identifier.
    async fn get_password_hash(&self, id: Id<User>) -> Result<Option<String>>;
}

#[async_trait]
impl<T> UserDataSource for &T
where
    T: UserDataSource + ?Sized,
{
    async fn create(&self, user: Self::Item, password_hash: String) -> Result<Self::Item> {
        (*self).create(user, password_hash).await
    }

    async fn read(&self, filter: UserFilters) -> Result<Vec<Self::Item>> {
        (*self).read(filter).await
    }

    async fn update(&self, user: Self::Item) -> Result<Option<Self::Item>> {
        (*self).update(user).await
    }

    async fn delete(&self, user: Self::Item) -> Result<Option<Self::Item>> {
        (*self).delete(user).await
    }

    async fn get_password_hash(&self, id: Id<User>) -> Result<Option<String>> {
        (*self).get_password_hash(id).await
    }
}

#[async_trait]
impl<T> UserDataSource for Arc<T>
where
    T: UserDataSource + ?Sized,
{
    async fn create(&self, user: Self::Item, password_hash: String) -> Result<Self::Item> {
        (**self).create(user, password_hash).await
    }

    async fn read(&self, filter: UserFilters) -> Result<Vec<Self::Item>> {
        (**self).read(filter).await
    }

    async fn update(&self, user: Self::Item) -> Result<Option<Self::Item>> {
        (**self).update(user).await
    }

    async fn delete(&self, user: Self::Item) -> Result<Option<Self::Item>> {
        (**self).delete(user).await
    }

    async fn get_password_hash(&self, id: Id<User>) -> Result<Option<String>> {
        (**self).get_password_hash(id).await
    }
}
