//! Data sources for users of the Flexible Project system.

use async_trait::async_trait;
use auto_impl::auto_impl;
use fp_core::model::id::Id;
use fp_core::model::user::{User, UserFilters};

use super::{DataSource, Result};

/// User data source type of the Flexible Project system.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait UserDataSource: DataSource<Item = User> {
    /// Create new user from [user data](User) and password hash
    /// which will be saved for this user.
    async fn create(&self, user: User, password_hash: String) -> Result<User>;

    /// Find users by provided [filters](UserFilters).
    async fn read(&self, filter: UserFilters) -> Result<Vec<User>>;

    /// Update user which has the same [identifier](fp_core::model::id::Id)
    /// from the user parameter with provided [user data](User).
    async fn update(&self, user: User) -> Result<Option<User>>;

    /// Delete user with the same data as in the user parameter.
    async fn delete(&self, user: User) -> Result<Option<User>>;

    /// Retrieve password hash from the user by its identifier.
    async fn get_password_hash(&self, id: Id<User>) -> Result<Option<String>>;

    /// Change password hash of the user by its identifier.
    async fn set_password_hash(&self, id: Id<User>, password_hash: String) -> Result<()>;
}
