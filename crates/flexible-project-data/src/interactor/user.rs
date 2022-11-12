//! User use case implementations of the Flexible Project system.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use fp_core::model::id::{Id, IdFilters};
use fp_core::model::user::{
    User, UserCredentials, UserFilters, UserRole, UserToken, UserTokenClaims, UsernameFilters,
};
use fp_core::use_case::error::InternalError;
use fp_core::use_case::hasher::{PasswordHashVerifier, PasswordHasher as _};
use fp_core::use_case::id::IdGenerator as _;
use fp_core::use_case::user::{
    CurrentUser as CoreCurrentUser, CurrentUserError, DeleteUser as CoreDeleteUser,
    DeleteUserError, FilterUsers as CoreFilterUsers, SignIn as CoreSignIn, SignInError,
    SignUp as CoreSignUp, SignUpError, UserTokenGenerator as CoreUserTokenGenerator,
};
use fp_core::use_case::verifier::{
    UserCredentialsState, UserCredentialsVerifier as CoreUserCredentialsVerifier,
    UserTokenVerifier as _,
};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::data_source::user::UserDataSource;
use crate::interactor::hasher::PasswordHasher;
use crate::interactor::id::IdGenerator;
use crate::interactor::token::{secret, UserTokenClaimsData};
use crate::interactor::verifier::{UserCredentialsVerifier, UserTokenVerifier};
use crate::repository::user::UserRepository;

/// Interactor used to generate new user token from claims.
#[derive(Debug, Clone, Default)]
pub struct UserTokenGenerator;

impl CoreUserTokenGenerator for UserTokenGenerator {
    fn generate(&self, claims: UserTokenClaims) -> Result<UserToken, InternalError> {
        let claims = UserTokenClaimsData {
            id: claims.id.to_string(),
            exp: Utc::now() + Duration::hours(1),
        };
        let header = &Header::default();
        let key = &EncodingKey::from_secret(secret().as_bytes());
        let token = encode(header, &claims, key).map_err(InternalError::new)?;
        let token = UserToken { token };
        Ok(token)
    }
}

/// Interactor used to register new user in the system.
pub struct SignUp<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
    password_hasher: Arc<PasswordHasher>,
    credentials_verifier: UserCredentialsVerifier,
    id_generator: IdGenerator,
    token_generator: UserTokenGenerator,
}

impl<S> SignUp<S>
where
    S: UserDataSource,
{
    /// Creates new sign up interactor.
    pub fn new(
        repository: Arc<UserRepository<S>>,
        password_hasher: Arc<PasswordHasher>,
        credentials_verifier: UserCredentialsVerifier,
        id_generator: IdGenerator,
        token_generator: UserTokenGenerator,
    ) -> Self {
        Self {
            repository,
            password_hasher,
            credentials_verifier,
            id_generator,
            token_generator,
        }
    }
}

#[async_trait]
impl<S> CoreSignUp for SignUp<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn sign_up(&self, credentials: UserCredentials) -> Result<UserToken, SignUpError> {
        match self.credentials_verifier.verify(&credentials)? {
            UserCredentialsState::Valid => (),
            UserCredentialsState::InvalidUsername => return Err(SignUpError::InvalidUsername),
            UserCredentialsState::InvalidPassword => return Err(SignUpError::InvalidPassword),
        };
        let repository = self.repository.as_ref();
        let user = User {
            id: self
                .id_generator
                .generate()
                .expect("should never fail because of `Infallible` error type, aka 'never' type")
                .with_owner(),
            name: credentials.name.clone(),
            display_name: credentials.name,
            email: None,
            role: UserRole::User,
        };
        let password_hash = self.password_hasher.hash(&credentials.password)?;
        // TODO what if user name already taken? move to repo error
        let user = repository
            .create(user, password_hash)
            .await
            .map_err(InternalError::new)?;
        let claims = UserTokenClaims { id: user.id };
        let token = self.token_generator.generate(claims)?;
        Ok(token)
    }
}

/// Interactor used to login existing user in the system.
pub struct SignIn<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
    password_hasher: Arc<PasswordHasher>,
    credentials_verifier: UserCredentialsVerifier,
    token_generator: UserTokenGenerator,
}

impl<S> SignIn<S>
where
    S: UserDataSource,
{
    /// Creates new sign in interactor.
    pub fn new(
        repository: Arc<UserRepository<S>>,
        password_hasher: Arc<PasswordHasher>,
        credentials_verifier: UserCredentialsVerifier,
        token_generator: UserTokenGenerator,
    ) -> Self {
        Self {
            repository,
            password_hasher,
            credentials_verifier,
            token_generator,
        }
    }
}

#[async_trait]
impl<S> CoreSignIn for SignIn<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn sign_in(&self, credentials: UserCredentials) -> Result<UserToken, SignInError> {
        match self.credentials_verifier.verify(&credentials)? {
            UserCredentialsState::Valid => (),
            UserCredentialsState::InvalidUsername => return Err(SignInError::InvalidUsername),
            UserCredentialsState::InvalidPassword => return Err(SignInError::InvalidPassword),
        };
        let repository = self.repository.as_ref();

        let filters = UserFilters::builder()
            .name(UsernameFilters::builder().eq(credentials.name).build())
            .build();
        let user = repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned()
            .ok_or(SignInError::NoUser)?;

        let password_hash = repository
            .get_password_hash(user.id.clone())
            .await
            .map_err(InternalError::new)?
            .ok_or(SignInError::NoUser)?;
        self.password_hasher
            .verify(&credentials.password, &password_hash)?
            .then_some(())
            .ok_or(SignInError::WrongPassword)?;

        let claims = UserTokenClaims { id: user.id };
        let token = self.token_generator.generate(claims)?;
        Ok(token)
    }
}

/// Interactor used to get current user from the token.
pub struct CurrentUser<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
    token_verifier: UserTokenVerifier,
}

impl<S> CurrentUser<S>
where
    S: UserDataSource,
{
    /// Creates new current user interactor.
    pub fn new(repository: Arc<UserRepository<S>>, token_verifier: UserTokenVerifier) -> Self {
        Self {
            repository,
            token_verifier,
        }
    }
}

#[async_trait]
impl<S> CoreCurrentUser for CurrentUser<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn current_user(&self, token: UserToken) -> Result<User, CurrentUserError> {
        let UserTokenClaims { id } = self.token_verifier.verify(&token)?;
        let repository = self.repository.as_ref();
        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(id).build())
            .build();
        let user = repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned()
            .ok_or(CurrentUserError::NoUser)?;
        Ok(user)
    }
}

impl<S> Clone for CurrentUser<S>
where
    S: UserDataSource,
{
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
            token_verifier: self.token_verifier.clone(),
        }
    }
}

/// Interactor used to delete user from the system.
pub struct DeleteUser<S>
where
    S: UserDataSource,
{
    repository: Arc<UserRepository<S>>,
    current_user: CurrentUser<S>,
}

impl<S> DeleteUser<S>
where
    S: UserDataSource,
{
    /// Creates new delete user interactor.
    pub fn new(repository: Arc<UserRepository<S>>, current_user: CurrentUser<S>) -> Self {
        Self {
            repository,
            current_user,
        }
    }
}

#[async_trait]
impl<S> CoreDeleteUser for DeleteUser<S>
where
    S: UserDataSource + Send + Sync,
{
    async fn delete(
        &self,
        token: UserToken,
        user_to_delete: Id<User>,
    ) -> Result<Option<User>, DeleteUserError> {
        let repository = self.repository.as_ref();
        let current_user = self.current_user.current_user(token).await?;
        if (current_user.id != user_to_delete) || current_user.role.is_user() {
            return Err(DeleteUserError::NotAllowed);
        }

        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(user_to_delete).build())
            .build();
        let user = repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned();
        let user = match user {
            Some(user) => user,
            None => return Ok(None),
        };
        let user = repository.delete(user).await.map_err(InternalError::new)?;
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
    async fn filter(&self, filters: UserFilters) -> Result<Vec<User>, InternalError> {
        let repository = self.repository.as_ref();
        let user = repository.read(filters).await.map_err(InternalError::new)?;
        Ok(user)
    }
}

impl<S> Clone for FilterUsers<S>
where
    S: UserDataSource,
{
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
        }
    }
}
