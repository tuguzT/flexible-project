use fancy_regex::Regex;
use fp_core::model::UserCredentials;
use fp_core::use_case::{
    PasswordVerifier as CorePasswordVerifier,
    UserCredentialsVerifier as CoreUserCredentialsVerifier,
    UsernameVerifier as CoreUsernameVerifier,
};
use once_cell::sync::Lazy;

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
    fn verify(&self, username: &str) -> bool {
        USERNAME_REGEX.is_match(username).unwrap_or_default()
    }
}

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"/^(?=.{4,32}$)(?![-_.])(?!.*[-_.]{2})[a-zA-Z\d\-_.]+(?<![-_.])$").unwrap()
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
    fn verify(&self, password: &str) -> bool {
        PASSWORD_REGEX.is_match(password).unwrap_or_default()
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
    fn verify<C>(&self, credentials: &C) -> bool
    where
        C: UserCredentials,
    {
        let UserCredentialsVerifier(uv, pv) = self;
        let username = credentials.name();
        let password = credentials.password();
        uv.verify(username) && pv.verify(password)
    }
}
