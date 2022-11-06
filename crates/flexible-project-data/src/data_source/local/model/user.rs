use fp_core::model::user::{User, UserRole};
use serde::{Deserialize, Serialize};

use crate::data_source::local::model::id::IdData;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct UserData {
    #[serde(rename = "_id")]
    pub id: IdData,
    pub name: String,
    pub display_name: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: UserRoleData,
}

impl From<UserData> for User {
    fn from(user: UserData) -> Self {
        Self {
            id: user.id.into(),
            name: user.name,
            display_name: user.display_name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize,
)]
pub enum UserRoleData {
    #[default]
    User,
    Moderator,
    Administrator,
}

impl From<UserRoleData> for UserRole {
    fn from(role: UserRoleData) -> Self {
        match role {
            UserRoleData::User => Self::User,
            UserRoleData::Moderator => Self::Moderator,
            UserRoleData::Administrator => Self::Administrator,
        }
    }
}

impl From<UserRole> for UserRoleData {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::User => Self::User,
            UserRole::Moderator => Self::Moderator,
            UserRole::Administrator => Self::Administrator,
        }
    }
}
