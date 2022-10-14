use std::sync::Arc;

use async_trait::async_trait;
use derive_more::{Display, Error, From};
use fp_core::model::{Id, User, UserCredentials, UserFilters, UserRole};
use fp_core::use_case::{
    CreateUser as CoreCreateUser, DeleteUser as CoreDeleteUser, FilterUsers as CoreFilterUsers,
    GUIDGenerator as CoreGUIDGenerator, UpdateUser as CoreUpdateUser,
    UserCredentialsVerifier as CoreUserCredentialsVerifier,
};

use crate::data_source::user::UserDataSource;
use crate::interactor::verifier::RegexError;
use crate::interactor::{GUIDGenerator, UserCredentialsVerifier};
use crate::repository::user::UserRepository;
use crate::repository::Error;

/// Interactor used to create new user in the system.
pub struct CreateUser<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
    verifier: UserCredentialsVerifier,
    id_generator: GUIDGenerator,
}

impl<S> CreateUser<S>
where
    S: UserDataSource,
{
    /// Creates new create user interactor.
    pub fn new(
        repository: Arc<UserRepository<S>>,
        verifier: UserCredentialsVerifier,
        id_generator: GUIDGenerator,
    ) -> Self {
        Self {
            repository,
            verifier,
            id_generator,
        }
    }
}

#[derive(Debug, Display, Error, From)]
#[from(forward)]
pub struct CreateUserError(#[error(source)] CreateUserErrorKind);

#[derive(Debug, Display, Error, From)]
enum CreateUserErrorKind {
    Repository(#[error(source)] Error),
    Regex(#[error(source)] RegexError),
    #[display(fmt = "user credentials does not match requirements")]
    UserCredentials,
}

#[async_trait]
impl<S> CoreCreateUser for CreateUser<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = CreateUserError;

    async fn create(&self, credentials: UserCredentials) -> Result<User, Self::Error> {
        self.verifier
            .verify(&credentials)?
            .then_some(())
            .ok_or(CreateUserErrorKind::UserCredentials)?;
        let repository = self.repository.as_ref();
        let id = self.id_generator.generate();
        let user = User {
            id: id.to_string().into(),
            name: credentials.name,
            email: None,
            role: UserRole::User,
        };
        let user = repository.create(user, credentials.password).await?;
        Ok(user)
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
    type Error = Error;

    async fn delete(&self, id: Id<User>) -> Result<Option<User>, Self::Error> {
        let repository = self.repository.as_ref();
        let filters = UserFilters { ids: vec![id] };
        let user = repository.read(filters).await?.first().cloned();
        let user = match user {
            Some(user) => user,
            None => return Ok(None),
        };
        let user = repository.delete(user).await?;
        Ok(user)
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
    type Error = Error;

    async fn filter(&self, filters: UserFilters) -> Result<Vec<User>, Self::Error> {
        let repository = self.repository.as_ref();
        let user = repository.read(filters).await?;
        Ok(user)
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
    type Error = Error;

    async fn update(&self, user: User) -> Result<Option<User>, Self::Error> {
        let repository = self.repository.as_ref();
        let user = repository.update(user).await?;
        Ok(user)
    }
}
