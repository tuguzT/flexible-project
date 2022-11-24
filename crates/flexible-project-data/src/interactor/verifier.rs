//! Verifier use case implementations of the Flexible Project system.

use async_trait::async_trait;
use fancy_regex::Regex;
use fp_core::model::user::{UserCredentials, UserToken, UserTokenClaims};
use fp_core::use_case::error::InternalError;
use fp_core::use_case::verifier::{
    PasswordVerifier as CorePasswordVerifier, UserCredentialsState,
    UserCredentialsVerifier as CoreUserCredentialsVerifier, UserTokenError,
    UserTokenVerifier as CoreUserTokenVerifier, UsernameVerifier as CoreUsernameVerifier,
};
use futures::join;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;
use tokio::task::spawn_blocking;

use crate::interactor::token::{secret, UserTokenClaimsData};

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
impl CoreUsernameVerifier for UsernameVerifier {
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
impl CorePasswordVerifier for PasswordVerifier {
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
#[derive(Debug, Clone, Default)]
pub struct UserCredentialsVerifier(UsernameVerifier, PasswordVerifier);

#[async_trait]
impl CoreUserCredentialsVerifier for UserCredentialsVerifier {
    async fn verify(
        &self,
        credentials: UserCredentials,
    ) -> Result<UserCredentialsState, InternalError> {
        let UserCredentialsVerifier(uv, pv) = self;
        let UserCredentials {
            password,
            name: username,
        } = credentials;

        let is_match = join!(uv.verify(username), pv.verify(password));
        let (is_match_username, is_match_password) = is_match;
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
#[derive(Debug, Clone, Default)]
pub struct UserTokenVerifier(());

#[async_trait]
impl CoreUserTokenVerifier for UserTokenVerifier {
    async fn verify(&self, token: UserToken) -> Result<UserTokenClaims, UserTokenError> {
        let token = token.token;
        let key = DecodingKey::from_secret(secret().as_bytes());
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
