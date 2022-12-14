//! Verifier use case implementations of the Flexible Project system.

use async_trait::async_trait;
use fancy_regex::Regex;
use fp_core::{
    model::user::{UserCredentials, UserToken, UserTokenClaims},
    use_case::{
        error::InternalError,
        verifier::{UserCredentialsState, UserTokenError},
    },
};
use futures::join;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;
use tokio::task::spawn_blocking;

use super::token::UserTokenClaimsData;

mod core {
    pub use fp_core::use_case::verifier::{
        PasswordVerifier, UserCredentialsVerifier, UserTokenVerifier, UsernameVerifier,
    };
}

/// Checks if username meets all the requirements.
///
/// These requirements are:
/// - must be from 4 to 32 characters in length;
/// - must contain latin or `-`, `_`, `.` characters;
/// - must not start or end with `-`, `_`, `.` characters;
/// - `-`, `_`, `.` characters can't be next to each other;
/// - `-`, `_`, `.` characters can't be used multiple times in a row.
#[derive(Debug, Clone, Default)]
pub struct UsernameVerifier(());

#[async_trait]
impl core::UsernameVerifier for UsernameVerifier {
    async fn verify(&self, username: String) -> Result<bool, InternalError> {
        let future = spawn_blocking(move || USERNAME_REGEX.is_match(&username));
        let result = future.await.map_err(InternalError::new)?;
        let is_match = result.map_err(InternalError::new)?;
        Ok(is_match)
    }
}

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?=.{4,32}$)(?![-_.])(?!.*[-_.]{2})[a-zA-Z\d\-_.]+(?<![-_.])$").unwrap()
});

/// Checks if password meets all the requirements.
///
/// These requirements are:
/// - must be from 8 characters in length;
/// - must contain at least one upper case latin letter;
/// - must contain at least one lower case latin letter;
/// - must contain at least one digit;
/// - must contain at least one special character: any of `()#?!@$%^&*_-`.
#[derive(Debug, Clone, Default)]
pub struct PasswordVerifier(());

#[async_trait]
impl core::PasswordVerifier for PasswordVerifier {
    async fn verify(&self, password: String) -> Result<bool, InternalError> {
        let future = spawn_blocking(move || PASSWORD_REGEX.is_match(&password));
        let result = future.await.map_err(InternalError::new)?;
        let is_match = result.map_err(InternalError::new)?;
        Ok(is_match)
    }
}

static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?\d)(?=.*?[()#?!@$%^&*_-]).{8,}$").unwrap()
});

/// Checks if user credentials meets all the requirements.
///
/// Requirements for username and password are listed in the documentation of
/// [username verifier](UsernameVerifier) and [password verifier](PasswordVerifier) accordingly.
#[derive(Debug, Clone)]
pub struct UserCredentialsVerifier<U, P>
where
    U: core::UsernameVerifier,
    P: core::PasswordVerifier,
{
    username_verifier: U,
    password_verifier: P,
}

impl<U, P> UserCredentialsVerifier<U, P>
where
    U: core::UsernameVerifier,
    P: core::PasswordVerifier,
{
    /// Creates new user credentials verifier interactor.
    pub fn new(username_verifier: U, password_verifier: P) -> Self {
        Self {
            username_verifier,
            password_verifier,
        }
    }
}

#[async_trait]
impl<U, P> core::UserCredentialsVerifier for UserCredentialsVerifier<U, P>
where
    U: core::UsernameVerifier,
    P: core::PasswordVerifier,
{
    async fn verify(
        &self,
        credentials: UserCredentials,
    ) -> Result<UserCredentialsState, InternalError> {
        let UserCredentials {
            password,
            name: username,
        } = credentials;

        let (is_match_username, is_match_password) = join!(
            self.username_verifier.verify(username),
            self.password_verifier.verify(password),
        );
        if !is_match_username? {
            return Ok(UserCredentialsState::InvalidUsername);
        }
        if !is_match_password? {
            return Ok(UserCredentialsState::InvalidPassword);
        }
        Ok(UserCredentialsState::Valid)
    }
}

/// Checks if user token was provided by client and generated by the server.
#[derive(Debug, Clone)]
pub struct UserTokenVerifier {
    secret: String,
}

impl UserTokenVerifier {
    /// Creates new user token verifier with provided secret.
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[async_trait]
impl core::UserTokenVerifier for UserTokenVerifier {
    async fn verify(&self, token: UserToken) -> Result<UserTokenClaims, UserTokenError> {
        let token = token.token;
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::default();

        let token_data =
            spawn_blocking(move || decode::<UserTokenClaimsData>(&token, &key, &validation))
                .await
                .map_err(InternalError::new)?
                .map_err(|error| match error.kind() {
                    ErrorKind::ExpiredSignature => UserTokenError::Expired,
                    _ => UserTokenError::Internal(InternalError::new(error)),
                })?;
        let claims = token_data.claims.into();
        Ok(claims)
    }
}
