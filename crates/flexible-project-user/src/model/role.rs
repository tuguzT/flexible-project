use derive_more::Display;
use fp_user_domain::model::Role as DomainRole;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn};

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

/// Filters for user role of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleFilters {
    /// Equality user role filter.
    pub eq: Option<Equal<Role>>,
    /// Inequality user role filter.
    pub ne: Option<NotEqual<Role>>,
    /// In user role filter.
    pub r#in: Option<In<Vec<Role>>>,
    /// Not in user role filter.
    pub nin: Option<NotIn<Vec<Role>>>,
}
