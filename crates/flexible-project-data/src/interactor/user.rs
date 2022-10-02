use std::sync::Arc;

use async_trait::async_trait;
use fp_core::model::{Node, UserCredentials, UserFilters, UserRole};
use fp_core::use_case::{
    CreateUser as CoreCreateUser, DeleteUser as CoreDeleteUser, FilterUsers as CoreFilterUsers,
    UpdateUser as CoreUpdateUser,
};

use crate::data_source::user::UserDataSource;
use crate::interactor::UserCredentialsVerifier;
use crate::model::{Id, User};
use crate::repository::ops::{DeleteById, ReadAll, Save};
use crate::repository::user::UserRepository;

/// Interactor used to create new user in the system.
pub struct CreateUser<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
    _verifier: UserCredentialsVerifier,
}

impl<S> CreateUser<S>
where
    S: UserDataSource,
{
    /// Creates new create user interactor.
    pub fn new(repository: Arc<UserRepository<S>>, verifier: UserCredentialsVerifier) -> Self {
        Self {
            repository,
            _verifier: verifier,
        }
    }
}

#[async_trait]
impl<S> CoreCreateUser for CreateUser<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = S::Error;

    type User = User;

    async fn create<C>(&self, credentials: &C) -> Result<Self::User, Self::Error>
    where
        C: UserCredentials + Sync,
    {
        let repository = self.repository.as_ref();
        let user = User {
            id: Id::random(),
            name: credentials.name().to_string(),
            email: None,
            role: UserRole::User,
        };
        repository.save(user).await
    }
}

/// Interactor used to delete user from the system.
pub struct DeleteUser<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
}

impl<S> DeleteUser<S>
where
    S: UserDataSource,
{
    /// Creates new delete user interactor.
    pub fn new(repository: Arc<UserRepository<S>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<S> CoreDeleteUser for DeleteUser<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = S::Error;

    type User = User;

    async fn delete(&self, id: <User as Node>::Id) -> Result<Option<Self::User>, Self::Error> {
        let repository = self.repository.as_ref();
        repository.delete_by_id(id).await
    }
}

/// Interactor used to filter users.
pub struct FilterUsers<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
}

impl<S> FilterUsers<S>
where
    S: UserDataSource,
{
    /// Creates new filter users predicate.
    pub fn new(repository: Arc<UserRepository<S>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<S> CoreFilterUsers for FilterUsers<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = S::Error;

    type User = User;

    async fn filter(&self, _filters: UserFilters) -> Result<Vec<Self::User>, Self::Error> {
        let repository = self.repository.as_ref();
        repository.read_all().await
    }
}

/// Interactor used to update users.
pub struct UpdateUser<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
}

impl<S> UpdateUser<S>
where
    S: UserDataSource,
{
    /// Creates user update interactor.
    pub fn new(repository: Arc<UserRepository<S>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<S> CoreUpdateUser for UpdateUser<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = S::Error;

    type User = User;

    async fn update(&self, user: Self::User) -> Result<Self::User, Self::Error> {
        let repository = self.repository.as_ref();
        repository.save(user).await
    }
}
