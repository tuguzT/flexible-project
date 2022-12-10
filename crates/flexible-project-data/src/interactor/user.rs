//! User use case implementations of the Flexible Project system.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use fp_core::{
    model::{
        id::{Id, IdFilters},
        user::{
            User, UserCredentials, UserFilters, UserRole, UserToken, UserTokenClaims,
            UsernameFilters,
        },
    },
    use_case::{
        error::InternalError,
        hasher::{PasswordHashVerifier, PasswordHasher},
        id::IdGenerator,
        user::{CurrentUserError, DeleteUserError, SignInError, SignUpError},
        verifier::{UserCredentialsState, UserCredentialsVerifier, UserTokenVerifier},
    },
};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::data_source::user::UserDataSource;
use crate::interactor::token::UserTokenClaimsData;
use crate::repository::user::UserRepository;

mod core {
    pub use fp_core::use_case::user::{
        CurrentUser, DeleteUser, FilterUsers, SignIn, SignUp, UserTokenGenerator,
    };
}

/// Interactor used to generate new user token from claims.
#[derive(Debug, Clone)]
pub struct UserTokenGenerator {
    secret: String,
}

impl UserTokenGenerator {
    /// Creates new user token generator with provided secret.
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[async_trait]
impl core::UserTokenGenerator for UserTokenGenerator {
    async fn generate(&self, claims: UserTokenClaims) -> Result<UserToken, InternalError> {
        let claims = UserTokenClaimsData {
            id: claims.id.to_string(),
            exp: Utc::now() + Duration::hours(1),
        };
        let header = &Header::default();
        let key = &EncodingKey::from_secret(self.secret.as_bytes());
        let token = encode(header, &claims, key).map_err(InternalError::new)?;
        let token = UserToken { token };
        Ok(token)
    }
}

/// Interactor used to register new user in the system.
pub struct SignUp {
    repository: UserRepository<Arc<dyn UserDataSource>>,
    password_hasher: Arc<dyn PasswordHasher>,
    credentials_verifier: Arc<dyn UserCredentialsVerifier>,
    id_generator: Arc<dyn IdGenerator>,
    token_generator: Arc<dyn core::UserTokenGenerator>,
}

impl SignUp {
    /// Creates new sign up interactor.
    pub fn new(
        repository: UserRepository<Arc<dyn UserDataSource>>,
        password_hasher: Arc<dyn PasswordHasher>,
        credentials_verifier: Arc<dyn UserCredentialsVerifier>,
        id_generator: Arc<dyn IdGenerator>,
        token_generator: Arc<dyn core::UserTokenGenerator>,
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
impl core::SignUp for SignUp {
    async fn sign_up(&self, credentials: UserCredentials) -> Result<UserToken, SignUpError> {
        match self
            .credentials_verifier
            .verify(credentials.clone())
            .await?
        {
            UserCredentialsState::Valid => (),
            UserCredentialsState::InvalidUsername => return Err(SignUpError::InvalidUsername),
            UserCredentialsState::InvalidPassword => return Err(SignUpError::InvalidPassword),
        };

        let username = credentials.name;
        let filters = UserFilters::builder()
            .name(UsernameFilters::builder().eq(username.clone()).build())
            .build();
        let username_taken = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .is_some();
        if username_taken {
            return Err(SignUpError::UsernameAlreadyTaken);
        }

        let user = User {
            id: self.id_generator.generate().await?.with_owner(),
            name: username.clone(),
            display_name: username,
            email: None,
            role: UserRole::User,
        };
        let password_hash = self.password_hasher.hash(credentials.password).await?;
        let user = self
            .repository
            .create(user, password_hash)
            .await
            .map_err(InternalError::new)?;
        let claims = UserTokenClaims { id: user.id };
        let token = self.token_generator.generate(claims).await?;
        Ok(token)
    }
}

/// Interactor used to login existing user in the system.
pub struct SignIn {
    repository: UserRepository<Arc<dyn UserDataSource>>,
    password_hash_verifier: Arc<dyn PasswordHashVerifier>,
    credentials_verifier: Arc<dyn UserCredentialsVerifier>,
    token_generator: Arc<dyn core::UserTokenGenerator>,
}

impl SignIn {
    /// Creates new sign in interactor.
    pub fn new(
        repository: UserRepository<Arc<dyn UserDataSource>>,
        password_hash_verifier: Arc<dyn PasswordHashVerifier>,
        credentials_verifier: Arc<dyn UserCredentialsVerifier>,
        token_generator: Arc<dyn core::UserTokenGenerator>,
    ) -> Self {
        Self {
            repository,
            password_hash_verifier,
            credentials_verifier,
            token_generator,
        }
    }
}

#[async_trait]
impl core::SignIn for SignIn {
    async fn sign_in(&self, credentials: UserCredentials) -> Result<UserToken, SignInError> {
        match self
            .credentials_verifier
            .verify(credentials.clone())
            .await?
        {
            UserCredentialsState::Valid => (),
            UserCredentialsState::InvalidUsername => return Err(SignInError::InvalidUsername),
            UserCredentialsState::InvalidPassword => return Err(SignInError::InvalidPassword),
        };

        let filters = UserFilters::builder()
            .name(UsernameFilters::builder().eq(credentials.name).build())
            .build();
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned()
            .ok_or(SignInError::NoUser)?;

        let password_hash = self
            .repository
            .get_password_hash(user.id.clone())
            .await
            .map_err(InternalError::new)?
            .ok_or(SignInError::NoUser)?;
        self.password_hash_verifier
            .verify(credentials.password, password_hash)
            .await?
            .then_some(())
            .ok_or(SignInError::WrongPassword)?;

        let claims = UserTokenClaims { id: user.id };
        let token = self.token_generator.generate(claims).await?;
        Ok(token)
    }
}

/// Interactor used to get current user from the token.
pub struct CurrentUser {
    repository: UserRepository<Arc<dyn UserDataSource>>,
    token_verifier: Arc<dyn UserTokenVerifier>,
}

impl CurrentUser {
    /// Creates new current user interactor.
    pub fn new(
        repository: UserRepository<Arc<dyn UserDataSource>>,
        token_verifier: Arc<dyn UserTokenVerifier>,
    ) -> Self {
        Self {
            repository,
            token_verifier,
        }
    }
}

#[async_trait]
impl core::CurrentUser for CurrentUser {
    async fn current_user(&self, token: UserToken) -> Result<User, CurrentUserError> {
        let UserTokenClaims { id } = self.token_verifier.verify(token).await?;
        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(id).build())
            .build();
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned()
            .ok_or(CurrentUserError::NoUser)?;
        Ok(user)
    }
}

/// Interactor used to delete user from the system.
pub struct DeleteUser {
    repository: UserRepository<Arc<dyn UserDataSource>>,
    current_user: Arc<dyn core::CurrentUser>,
}

impl DeleteUser {
    /// Creates new delete user interactor.
    pub fn new(
        repository: UserRepository<Arc<dyn UserDataSource>>,
        current_user: Arc<dyn core::CurrentUser>,
    ) -> Self {
        Self {
            repository,
            current_user,
        }
    }
}

#[async_trait]
impl core::DeleteUser for DeleteUser {
    async fn delete(
        &self,
        token: UserToken,
        user_to_delete: Id<User>,
    ) -> Result<Option<User>, DeleteUserError> {
        let current_user = self.current_user.current_user(token).await?;
        if (current_user.id != user_to_delete) || current_user.role.is_user() {
            return Err(DeleteUserError::NotAllowed);
        }

        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(user_to_delete).build())
            .build();
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned();
        let user = match user {
            Some(user) => user,
            None => return Ok(None),
        };
        let user = self
            .repository
            .delete(user)
            .await
            .map_err(InternalError::new)?;
        Ok(user)
    }
}

/// Interactor used to filter users.
pub struct FilterUsers {
    repository: UserRepository<Arc<dyn UserDataSource>>,
}

impl FilterUsers {
    /// Creates new filter users predicate.
    pub fn new(repository: UserRepository<Arc<dyn UserDataSource>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl core::FilterUsers for FilterUsers {
    async fn filter(&self, filters: UserFilters) -> Result<Vec<User>, InternalError> {
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?;
        Ok(user)
    }
}
