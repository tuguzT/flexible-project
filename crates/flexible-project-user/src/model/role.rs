use std::borrow::Cow;

use derive_more::Display;
use fp_user_domain::model::{Role as DomainRole, RoleFilters as DomainRoleFilters};
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

impl From<DomainRoleFilters<'_>> for RoleFilters {
    fn from(filters: DomainRoleFilters<'_>) -> Self {
        let DomainRoleFilters { eq, ne, r#in, nin } = filters;
        Self {
            eq: eq.map(|role| Equal(role.0.into_owned().into())),
            ne: ne.map(|role| NotEqual(role.0.into_owned().into())),
            r#in: r#in.map(|r#in| In(r#in.0.iter().cloned().map(Into::into).collect())),
            nin: nin.map(|r#in| NotIn(r#in.0.iter().cloned().map(Into::into).collect())),
        }
    }
}

impl From<RoleFilters> for DomainRoleFilters<'_> {
    fn from(filters: RoleFilters) -> Self {
        let RoleFilters { eq, ne, r#in, nin } = filters;
        Self {
            eq: eq.map(|Equal(role)| Equal(Cow::Owned(role.into())).into()),
            ne: ne.map(|NotEqual(role)| NotEqual(Cow::Owned(role.into())).into()),
            r#in: r#in.map(|In(roles)| {
                let roles: Vec<_> = roles.into_iter().map(Into::into).collect();
                In(roles.into()).into()
            }),
            nin: nin.map(|NotIn(roles)| {
                let roles: Vec<_> = roles.into_iter().map(Into::into).collect();
                NotIn(roles.into()).into()
            }),
        }
    }
}
