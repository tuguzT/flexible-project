use derive_more::{Display, Error, From};
use fancy_regex::Regex;
use fp_core::model::{UserCredentials, UserToken, UserTokenClaims};
use fp_core::use_case::{
    PasswordVerifier as CorePasswordVerifier, UserCredentialsState,
    UserCredentialsVerifier as CoreUserCredentialsVerifier,
    UserTokenVerifier as CoreUserTokenVerifier, UsernameVerifier as CoreUsernameVerifier,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;

use crate::interactor::token::{secret, JwtError, UserTokenClaimsData};

#[derive(Debug, Display, Error, From)]
pub struct RegexError(#[error(source)] fancy_regex::Error);

/// Checks if username meets all the requirements.
///
/// These requirements are:
/// - must be from 4 to 32 characters in length;
/// - must contain latin or `-`, `_`, `.` characters;
/// - must not start or end with `-`, `_`, `.` characters;
/// - `-`, `_`, `.` characters can't be next to each other;
/// - `-`, `_`, `.` characters can't be used multiple times in a row.
#[derive(Default, Clone, Copy)]
pub struct UsernameVerifier;

impl CoreUsernameVerifier for UsernameVerifier {
    type Error = RegexError;

    fn verify(&self, username: &str) -> Result<bool, Self::Error> {
        let is_match = USERNAME_REGEX.is_match(username)?;
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
#[derive(Default, Clone, Copy)]
pub struct PasswordVerifier;

impl CorePasswordVerifier for PasswordVerifier {
    type Error = RegexError;

    fn verify(&self, password: &str) -> Result<bool, Self::Error> {
        let is_match = PASSWORD_REGEX.is_match(password)?;
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
#[derive(Default, Clone, Copy)]
pub struct UserCredentialsVerifier(UsernameVerifier, PasswordVerifier);

impl CoreUserCredentialsVerifier for UserCredentialsVerifier {
    type Error = RegexError;

    fn verify(&self, credentials: &UserCredentials) -> Result<UserCredentialsState, Self::Error> {
        let UserCredentialsVerifier(uv, pv) = self;
        let username = &credentials.name;
        if !uv.verify(username)? {
            return Ok(UserCredentialsState::InvalidUsername);
        }
        let password = &credentials.password;
        if !pv.verify(password)? {
            return Ok(UserCredentialsState::InvalidPassword);
        }
        Ok(UserCredentialsState::Valid)
    }
}

/// Checks if user token was provided by client and generated by the server.
#[derive(Default, Clone, Copy)]
pub struct UserTokenVerifier;

impl CoreUserTokenVerifier for UserTokenVerifier {
    type Error = JwtError;

    fn verify(&self, token: &UserToken) -> Result<UserTokenClaims, Self::Error> {
        let token = &token.token;
        let key = &DecodingKey::from_secret(secret());
        let validation = &Validation::default();
        let token_data = decode::<UserTokenClaimsData>(token, key, validation)?;
        let claims = token_data.claims.into();
        Ok(claims)
    }
}
