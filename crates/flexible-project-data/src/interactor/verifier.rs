use derive_more::{Display, Error, From};
use fancy_regex::Regex;
use fp_core::model::UserCredentials;
use fp_core::use_case::{
    PasswordVerifier as CorePasswordVerifier,
    UserCredentialsVerifier as CoreUserCredentialsVerifier,
    UsernameVerifier as CoreUsernameVerifier,
};
use once_cell::sync::Lazy;

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
#[derive(Default)]
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
#[derive(Default)]
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
#[derive(Default)]
pub struct UserCredentialsVerifier(UsernameVerifier, PasswordVerifier);

impl CoreUserCredentialsVerifier for UserCredentialsVerifier {
    type Error = RegexError;

    fn verify(&self, credentials: &UserCredentials) -> Result<bool, Self::Error> {
        let UserCredentialsVerifier(uv, pv) = self;
        let username = &credentials.name;
        let password = &credentials.password;
        let is_match = uv.verify(username)? && pv.verify(password)?;
        Ok(is_match)
    }
}
