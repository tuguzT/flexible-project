use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use derive_more::{Display, Error, From};
use fp_core::model::UserTokenClaims;
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error, From)]
pub struct JwtError(#[error(source)] Error);

#[derive(Deserialize, Serialize)]
pub struct UserTokenClaimsData {
    pub id: String,
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

impl From<UserTokenClaimsData> for UserTokenClaims {
    fn from(claims: UserTokenClaimsData) -> Self {
        let id = claims.id.into();
        Self { id }
    }
}

pub fn secret() -> &'static [u8] {
    "secret".as_bytes()
}
