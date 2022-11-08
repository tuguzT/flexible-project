//! User use case implementations of the Flexible Project system.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use derive_more::{Display, Error, From};
use fp_core::model::id::{Id, IdFilters};
use fp_core::model::user::{
    User, UserCredentials, UserFilters, UserRole, UserToken, UserTokenClaims, UsernameFilters,
};
use fp_core::use_case::hasher::{PasswordHashVerifier, PasswordHasher as _};
use fp_core::use_case::id::IdGenerator as _;
use fp_core::use_case::user::{
    DeleteUser as CoreDeleteUser, FilterUsers as CoreFilterUsers, SignIn as CoreSignIn,
    SignUp as CoreSignUp, UpdateUser as CoreUpdateUser,
    UserTokenGenerator as CoreUserTokenGenerator,
};
use fp_core::use_case::verifier::{
    UserCredentialsState, UserCredentialsVerifier as CoreUserCredentialsVerifier,
};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::data_source::user::UserDataSource;
use crate::interactor::hasher::{PasswordHashError, PasswordHashVerifyError, PasswordHasher};
use crate::interactor::id::IdGenerator;
use crate::interactor::token::{secret, JwtError, UserTokenClaimsData};
use crate::interactor::verifier::{RegexError, UserCredentialsVerifier};
use crate::repository::user::UserRepository;
use crate::repository::Error;

/// Interactor used to generate new user token from claims.
#[derive(Default, Clone, Copy)]
pub struct UserTokenGenerator;

impl CoreUserTokenGenerator for UserTokenGenerator {
    type Error = JwtError;

    fn generate(&self, claims: UserTokenClaims) -> Result<UserToken, Self::Error> {
        let claims = UserTokenClaimsData {
            id: claims.id.to_string(),
            exp: Utc::now() + Duration::hours(1),
        };
        let header = &Header::default();
        let key = &EncodingKey::from_secret(secret().as_slice());
        let token = encode(header, &claims, key).map_err(JwtError::from)?;
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

/// Error type of sign up use case.
#[derive(Debug, Display, Error, From)]
pub enum SignUpError {
    /// Repository error variant.
    Repository(Error),
    /// Regex execution error variant.
    Regex(RegexError),
    /// JWT token verification error variant.
    Jwt(JwtError),
    /// Password hashing error variant.
    PasswordHash(PasswordHashError),
    /// Invalid username error variant.
    #[display(fmt = "user name does not match requirements")]
    InvalidUsername,
    /// Invalid password error variant.
    #[display(fmt = "user password does not match requirements")]
    InvalidPassword,
}

#[async_trait]
impl<S> CoreSignUp for SignUp<S>
where
    S: UserDataSource + Send + Sync,
{
    type Error = SignUpError;

    async fn sign_up(&self, credentials: UserCredentials) -> Result<UserToken, Self::Error> {
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
        let user = repository.create(user, password_hash).await?;
        let claims = UserTokenClaims { id: user.id };
        let token = self.token_generator.generate(claims)?;
        Ok(token)
    }
}

/// Error type of sign in use case.
#[derive(Debug, Display, From, Error)]
pub enum SignInError {
    /// Repository error variant.
    Repository(Error),
    /// Regex execution error variant.
    Regex(RegexError),
    /// JWT token verification error variant.
    Jwt(JwtError),
    /// Password verification error variant.
    PasswordVerify(PasswordHashVerifyError),
    /// Invalid username error variant.
    #[display(fmt = "user name does not match requirements")]
    InvalidUsername,
    /// Invalid password error variant.
    #[display(fmt = "user password does not match requirements")]
    InvalidPassword,
    /// User password is wrong.
    #[display(fmt = "wrong password")]
    WrongPassword,
    /// No user was found by credentials.
    #[display(fmt = "no user was found")]
    NoUser,
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
    type Error = SignInError;

    async fn sign_in(&self, credentials: UserCredentials) -> Result<UserToken, Self::Error> {
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
            .await?
            .first()
            .cloned()
            .ok_or(SignInError::NoUser)?;

        let password_hash = repository
            .get_password_hash(user.id.clone())
            .await?
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
        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(id).build())
            .build();
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
