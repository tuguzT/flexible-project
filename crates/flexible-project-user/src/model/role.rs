use derive_more::Display;
use fp_user_domain::model::Role as DomainRole;
use serde::{Deserialize, Serialize};

/// Serializable [role](DomainRole) of the user.
#[derive(
    Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum Role {
    /// An ordinary user with no special rights.
    User,
    /// A moderator of the system which is responsible
    /// for public user content moderation.
    Moderator,
    /// An administrator of the system with special rights.
    Administrator,
}

impl From<DomainRole> for Role {
    fn from(role: DomainRole) -> Self {
        match role {
            DomainRole::User => Self::User,
            DomainRole::Moderator => Self::Moderator,
            DomainRole::Administrator => Self::Administrator,
        }
    }
}

impl From<Role> for DomainRole {
    fn from(role: Role) -> Self {
        match role {
            Role::User => Self::User,
            Role::Moderator => Self::Moderator,
            Role::Administrator => Self::Administrator,
        }
    }
}
