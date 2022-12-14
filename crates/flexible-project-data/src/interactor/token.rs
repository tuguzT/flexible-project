//! Token use case implementations of the Flexible Project system.

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use fp_core::model::user::UserTokenClaims;
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
