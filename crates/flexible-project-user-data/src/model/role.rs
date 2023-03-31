use derive_more::Display;
use fp_user_domain::model::Role;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
)]
pub enum LocalRole {
    #[default]
    User,
    Moderator,
    Administrator,
}

impl From<Role> for LocalRole {
    fn from(value: Role) -> Self {
        match value {
            Role::User => Self::User,
            Role::Moderator => Self::Moderator,
            Role::Administrator => Self::Administrator,
        }
    }
}

impl From<LocalRole> for Role {
    fn from(value: LocalRole) -> Self {
        match value {
            LocalRole::User => Self::User,
            LocalRole::Moderator => Self::Moderator,
            LocalRole::Administrator => Self::Administrator,
        }
    }
}
