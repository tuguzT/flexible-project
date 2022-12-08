//! Token use case implementations of the Flexible Project system.

use std::env;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use fp_core::model::user::UserTokenClaims;
use fp_core::use_case::error::{BoxedError, InternalError};
use serde::{Deserialize, Serialize};

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

pub fn secret() -> Result<String, InternalError> {
    env::var("JWT_SECRET").map_err(|_| {
        let error = "JWT_SECRET environment variable must be set".to_string();
        InternalError::from(BoxedError::from(error))
    })
}
