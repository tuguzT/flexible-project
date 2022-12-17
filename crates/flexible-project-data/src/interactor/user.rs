//! User use case implementations of the Flexible Project system.

use async_trait::async_trait;
use chrono::{Duration, Utc};
use derive_more::{Display, Error};
use fp_core::{
    model::{
        id::{Id, IdFilters},
        user::{
            User, UserCredentials, UserEmailFilters, UserFilters, UserRole, UserToken,
            UserTokenClaims, UsernameFilters,
        },
    },
    use_case::{
        error::InternalError,
        hasher::{PasswordHashVerifier, PasswordHasher},
        id::IdGenerator,
        user::{
            CurrentUserError, DeleteUserError, GrantUserRoleError, SignInError, SignUpError,
            UpdateUserDisplayNameError, UpdateUserEmailError, UpdateUserPasswordError,
            UpdateUsernameError,
        },
        verifier::{
            PasswordVerifier, UserCredentialsState, UserCredentialsVerifier, UserTokenVerifier,
            UsernameVerifier,
        },
    },
};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::data_source::user::UserDataSource;
use crate::interactor::token::UserTokenClaimsData;
use crate::repository::user::UserRepository;

mod core {
    pub use fp_core::use_case::user::{
        CurrentUser, DeleteUser, FilterUsers, GrantUserRole, SignIn, SignUp, UpdateUserDisplayName,
        UpdateUserEmail, UpdateUserPassword, UpdateUsername, UserTokenGenerator,
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
pub struct SignUp<U, P, C, I, T>
where
    U: UserDataSource,
    P: PasswordHasher,
    C: UserCredentialsVerifier,
    I: IdGenerator,
    T: core::UserTokenGenerator,
{
    repository: UserRepository<U>,
    password_hasher: P,
    credentials_verifier: C,
    id_generator: I,
    token_generator: T,
}

impl<U, P, C, I, T> SignUp<U, P, C, I, T>
where
    U: UserDataSource,
    P: PasswordHasher,
    C: UserCredentialsVerifier,
    I: IdGenerator,
    T: core::UserTokenGenerator,
{
    /// Creates new sign up interactor.
    pub fn new(
        repository: UserRepository<U>,
        password_hasher: P,
        credentials_verifier: C,
        id_generator: I,
        token_generator: T,
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
impl<U, P, C, I, T> core::SignUp for SignUp<U, P, C, I, T>
where
    U: UserDataSource,
    P: PasswordHasher,
    C: UserCredentialsVerifier,
    I: IdGenerator,
    T: core::UserTokenGenerator,
{
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
pub struct SignIn<U, P, C, T>
where
    U: UserDataSource,
    P: PasswordHashVerifier,
    C: UserCredentialsVerifier,
    T: core::UserTokenGenerator,
{
    repository: UserRepository<U>,
    password_hash_verifier: P,
    credentials_verifier: C,
    token_generator: T,
}

impl<U, P, C, T> SignIn<U, P, C, T>
where
    U: UserDataSource,
    P: PasswordHashVerifier,
    C: UserCredentialsVerifier,
    T: core::UserTokenGenerator,
{
    /// Creates new sign in interactor.
    pub fn new(
        repository: UserRepository<U>,
        password_hash_verifier: P,
        credentials_verifier: C,
        token_generator: T,
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
impl<U, P, C, T> core::SignIn for SignIn<U, P, C, T>
where
    U: UserDataSource,
    P: PasswordHashVerifier,
    C: UserCredentialsVerifier,
    T: core::UserTokenGenerator,
{
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
pub struct CurrentUser<U, T>
where
    U: UserDataSource,
    T: UserTokenVerifier,
{
    repository: UserRepository<U>,
    token_verifier: T,
}

impl<U, T> CurrentUser<U, T>
where
    U: UserDataSource,
    T: UserTokenVerifier,
{
    /// Creates new current user interactor.
    pub fn new(repository: UserRepository<U>, token_verifier: T) -> Self {
        Self {
            repository,
            token_verifier,
        }
    }
}

#[async_trait]
impl<U, T> core::CurrentUser for CurrentUser<U, T>
where
    U: UserDataSource,
    T: UserTokenVerifier,
{
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
pub struct DeleteUser<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    repository: UserRepository<U>,
    current_user: C,
}

impl<U, C> DeleteUser<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    /// Creates new delete user interactor.
    pub fn new(repository: UserRepository<U>, current_user: C) -> Self {
        Self {
            repository,
            current_user,
        }
    }
}

#[async_trait]
impl<U, C> core::DeleteUser for DeleteUser<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
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
pub struct FilterUsers<U>
where
    U: UserDataSource,
{
    repository: UserRepository<U>,
}

impl<U> FilterUsers<U>
where
    U: UserDataSource,
{
    /// Creates new filter users interactor.
    pub fn new(repository: UserRepository<U>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<U> core::FilterUsers for FilterUsers<U>
where
    U: UserDataSource,
{
    async fn filter(&self, filters: UserFilters) -> Result<Vec<User>, InternalError> {
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?;
        Ok(user)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "user should present")]
struct NoUserError;

/// Interactor used to update user display name.
pub struct UpdateUserDisplayName<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    repository: UserRepository<U>,
    current_user: C,
}

impl<U, C> UpdateUserDisplayName<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    /// Creates new update user display name interactor.
    pub fn new(repository: UserRepository<U>, current_user: C) -> Self {
        Self {
            repository,
            current_user,
        }
    }
}

#[async_trait]
impl<U, C> core::UpdateUserDisplayName for UpdateUserDisplayName<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    async fn update_display_name(
        &self,
        token: UserToken,
        display_name: String,
    ) -> Result<User, UpdateUserDisplayNameError> {
        let current_user = self.current_user.current_user(token).await?;
        let user = User {
            display_name,
            ..current_user
        };
        let user = self
            .repository
            .update(user)
            .await
            .map_err(InternalError::new)?
            .ok_or(NoUserError)
            .map_err(InternalError::new)?;
        Ok(user)
    }
}

/// Interactor used to update user email.
pub struct UpdateUserEmail<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    repository: UserRepository<U>,
    current_user: C,
}

impl<U, C> UpdateUserEmail<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    /// Creates new update user email interactor.
    pub fn new(repository: UserRepository<U>, current_user: C) -> Self {
        Self {
            repository,
            current_user,
        }
    }
}

#[async_trait]
impl<U, C> core::UpdateUserEmail for UpdateUserEmail<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    async fn update_email(
        &self,
        token: UserToken,
        email: Option<String>,
    ) -> Result<User, UpdateUserEmailError> {
        let current_user = self.current_user.current_user(token).await?;

        if let Some(email) = &email {
            let filters = UserFilters::builder()
                .email(UserEmailFilters::builder().eq(email.clone()).build())
                .build();
            let user = self
                .repository
                .read(filters)
                .await
                .map_err(InternalError::new)?
                .first()
                .cloned();
            if user.is_some() {
                return Err(UpdateUserEmailError::AlreadyTaken);
            }
        }

        let user = User {
            email,
            ..current_user
        };
        let user = self
            .repository
            .update(user)
            .await
            .map_err(InternalError::new)?
            .ok_or(NoUserError)
            .map_err(InternalError::new)?;
        Ok(user)
    }
}

/// Interactor used to grant role to the user.
pub struct GrantUserRole<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    repository: UserRepository<U>,
    current_user: C,
}

impl<U, C> GrantUserRole<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    /// Creates new grant user role interactor.
    pub fn new(repository: UserRepository<U>, current_user: C) -> Self {
        Self {
            repository,
            current_user,
        }
    }
}

#[async_trait]
impl<U, C> core::GrantUserRole for GrantUserRole<U, C>
where
    U: UserDataSource,
    C: core::CurrentUser,
{
    async fn grant_role(
        &self,
        token: UserToken,
        user_to_grant: Id<User>,
        role: UserRole,
    ) -> Result<(), GrantUserRoleError> {
        let current_user = self.current_user.current_user(token).await?;
        if current_user.role != UserRole::Administrator {
            return Err(GrantUserRoleError::NotAllowed);
        }

        let filters = UserFilters::builder()
            .id(IdFilters::builder().eq(user_to_grant).build())
            .build();
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned();
        let Some(user) = user else {
            return Err(GrantUserRoleError::NoUserToGrant);
        };

        let user = User { role, ..user };
        let _ = self
            .repository
            .update(user)
            .await
            .map_err(InternalError::new)?
            .ok_or(NoUserError)
            .map_err(InternalError::new)?;
        Ok(())
    }
}

/// Interactor used to update user name.
pub struct UpdateUsername<U, V, C>
where
    U: UserDataSource,
    V: UsernameVerifier,
    C: core::CurrentUser,
{
    repository: UserRepository<U>,
    username_verifier: V,
    current_user: C,
}

impl<U, V, C> UpdateUsername<U, V, C>
where
    U: UserDataSource,
    V: UsernameVerifier,
    C: core::CurrentUser,
{
    /// Creates new update username interactor.
    pub fn new(repository: UserRepository<U>, username_verifier: V, current_user: C) -> Self {
        Self {
            repository,
            username_verifier,
            current_user,
        }
    }
}

#[async_trait]
impl<U, V, C> core::UpdateUsername for UpdateUsername<U, V, C>
where
    U: UserDataSource,
    V: UsernameVerifier,
    C: core::CurrentUser,
{
    async fn update_name(
        &self,
        token: UserToken,
        name: String,
    ) -> Result<User, UpdateUsernameError> {
        let current_user = self.current_user.current_user(token).await?;

        let verify = self.username_verifier.verify(name.clone()).await?;
        if !verify {
            return Err(UpdateUsernameError::InvalidUsername);
        }

        let filters = UserFilters::builder()
            .name(UsernameFilters::builder().eq(name.clone()).build())
            .build();
        let user = self
            .repository
            .read(filters)
            .await
            .map_err(InternalError::new)?
            .first()
            .cloned();
        if user.is_some() {
            return Err(UpdateUsernameError::AlreadyTaken);
        }

        let user = User {
            name,
            ..current_user
        };
        let user = self
            .repository
            .update(user)
            .await
            .map_err(InternalError::new)?
            .ok_or(NoUserError)
            .map_err(InternalError::new)?;
        Ok(user)
    }
}

/// Interactor used to update user password.
pub struct UpdateUserPassword<U, V, H, HV, C>
where
    U: UserDataSource,
    V: PasswordVerifier,
    H: PasswordHasher,
    HV: PasswordHashVerifier,
    C: core::CurrentUser,
{
    repository: UserRepository<U>,
    password_verifier: V,
    password_hasher: H,
    password_hash_verifier: HV,
    current_user: C,
}

impl<U, V, H, HV, C> UpdateUserPassword<U, V, H, HV, C>
where
    U: UserDataSource,
    V: PasswordVerifier,
    H: PasswordHasher,
    HV: PasswordHashVerifier,
    C: core::CurrentUser,
{
    /// Creates new update user password interactor.
    pub fn new(
        repository: UserRepository<U>,
        password_verifier: V,
        password_hasher: H,
        password_hash_verifier: HV,
        current_user: C,
    ) -> Self {
        Self {
            repository,
            password_verifier,
            password_hasher,
            password_hash_verifier,
            current_user,
        }
    }
}

#[async_trait]
impl<U, V, H, HV, C> core::UpdateUserPassword for UpdateUserPassword<U, V, H, HV, C>
where
    U: UserDataSource,
    V: PasswordVerifier,
    H: PasswordHasher,
    HV: PasswordHashVerifier,
    C: core::CurrentUser,
{
    async fn update_password(
        &self,
        token: UserToken,
        old_password: String,
        new_password: String,
    ) -> Result<(), UpdateUserPasswordError> {
        let current_user = self.current_user.current_user(token).await?;

        if old_password == new_password {
            return Err(UpdateUserPasswordError::SamePassword);
        }

        let verify = self.password_verifier.verify(old_password.clone()).await?;
        if !verify {
            return Err(UpdateUserPasswordError::InvalidPassword);
        }
        let verify = self.password_verifier.verify(new_password.clone()).await?;
        if !verify {
            return Err(UpdateUserPasswordError::InvalidPassword);
        }

        let password_hash = self
            .repository
            .get_password_hash(current_user.id.clone())
            .await
            .map_err(InternalError::new)?
            .ok_or(NoUserError)
            .map_err(InternalError::new)?;
        let verify = self
            .password_hash_verifier
            .verify(old_password, password_hash)
            .await?;
        if !verify {
            return Err(UpdateUserPasswordError::WrongPassword);
        }

        let password_hash = self.password_hasher.hash(new_password).await?;
        self.repository
            .set_password_hash(current_user.id, password_hash)
            .await
            .map_err(InternalError::new)?;
        Ok(())
    }
}
