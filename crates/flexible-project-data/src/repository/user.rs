//! Repositories for users of the Flexible Project system.

use fp_core::model::id::Id;
use fp_core::model::user::{User, UserFilters};

use crate::data_source::user::UserDataSource;

use super::{Repository, Result};

/// User repository of the Flexible Project system.
#[derive(Debug, Clone)]
pub struct UserRepository<S>(pub S)
where
    S: UserDataSource;

impl<S> UserRepository<S>
where
    S: UserDataSource,
{
    /// Create new user from [user data](User) and password hash
    /// which will be saved for this user.
    pub async fn create(&self, user: User, password_hash: String) -> Result<User> {
        let user = self.0.create(user, password_hash).await?;
        Ok(user)
    }

    /// Find users by provided [filters](UserFilters).
    pub async fn read(&self, filter: UserFilters) -> Result<Vec<User>> {
        let users = self.0.read(filter).await?;
        Ok(users)
    }

    /// Update user which has the same [identifier](fp_core::model::id::Id)
    /// from the user parameter with provided [user data](User).
    pub async fn update(&self, user: User) -> Result<Option<User>> {
        let user = self.0.update(user).await?;
        Ok(user)
    }

    /// Delete user with the same data as in the user parameter.
    pub async fn delete(&self, user: User) -> Result<Option<User>> {
        let user = self.0.delete(user).await?;
        Ok(user)
    }

    /// Retrieve password hash from the user by its identifier.
    pub async fn get_password_hash(&self, id: Id<User>) -> Result<Option<String>> {
        let password_hash = self.0.get_password_hash(id).await?;
        Ok(password_hash)
    }

    /// Change password hash of the user by its identifier.
    pub async fn set_password_hash(&self, id: Id<User>, password_hash: String) -> Result<()> {
        self.0.set_password_hash(id, password_hash).await?;
        Ok(())
    }
}

impl<S> Repository for UserRepository<S>
where
    S: UserDataSource,
{
    type Item = S::Item;
}
