use derive_more::{Display, Error, From};
use domain::model::UserId;
use mongodb::bson::uuid::{Error, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LocalUserId {
    inner: Uuid,
}

impl LocalUserId {
    pub(crate) fn new() -> Self {
        let inner = Uuid::new();
        Self { inner }
    }
}

impl TryFrom<UserId> for LocalUserId {
    type Error = LocalUserIdError;

    fn try_from(value: UserId) -> Result<Self, Self::Error> {
        let id = value.into_inner();
        let inner = Uuid::parse_str(id)?;
        Ok(Self { inner })
    }
}

impl From<LocalUserId> for UserId {
    fn from(value: LocalUserId) -> Self {
        let LocalUserId { inner } = value;
        let id = inner.to_string();
        Self::new(id)
    }
}

#[derive(Debug, Display, Clone, From, Error)]
pub struct LocalUserIdError(Error);
