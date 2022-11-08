//! Token use case implementations of the Flexible Project system.

use std::env;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use derive_more::{Display, Error, From};
use fp_core::model::user::UserTokenClaims;
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};

/// Error type which is returned on JWT token verification failure.
#[derive(Debug, Display, Error, From)]
pub struct JwtError(Error);

/// Data of the actual user claims stored inside of the token.
#[derive(Deserialize, Serialize)]
pub struct UserTokenClaimsData {
    /// Identifier representation.
    pub id: String,
    /// Time after which token will be treated as expired.
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

impl From<UserTokenClaimsData> for UserTokenClaims {
    fn from(claims: UserTokenClaimsData) -> Self {
        let id = claims.id.into();
        Self { id }
    }
}

pub fn secret() -> Vec<u8> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET environment variable should be set");
    log::debug!("JWT_SECRET is {}", &secret);
    secret.into_bytes()
}
