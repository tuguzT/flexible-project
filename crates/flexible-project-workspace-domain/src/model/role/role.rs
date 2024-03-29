#![allow(clippy::module_inception)]

use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_filter::Filter;
use typed_builder::TypedBuilder;

use super::{
    access::{RoleAccessLevel, RoleAccessLevelFilters},
    id::{RoleId, RoleIdFilters},
    name::{RoleName, RoleNameFilters},
};

/// Role of a member of the workspace in the system.
#[derive(Debug, Clone)]
pub struct Role {
    /// Unique identifier of the workspace role.
    pub id: RoleId,
    /// Name of the workspace role.
    ///
    /// Name **must** be unique in the scope of one workspace.
    pub name: RoleName,
    /// Access level of the workspace role.
    pub access_level: RoleAccessLevel,
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Role {}

impl Hash for Role {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Filters for workspace role of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleFilters<'a> {
    /// Workspace role identifier filters.
    pub id: Option<RoleIdFilters<'a>>,
    /// Workspace role name filters.
    pub name: Option<RoleNameFilters<'a>>,
    /// Workspace role access level filters.
    pub access_level: Option<RoleAccessLevelFilters<'a>>,
}

impl<Input> Filter<Input> for RoleFilters<'_>
where
    Input: Borrow<Role>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            id: id_filter,
            name: name_filter,
            access_level: access_level_filter,
        } = self;
        let Role {
            id,
            name,
            access_level,
        } = input.borrow();
        id_filter.satisfies(id)
            && name_filter.satisfies(name)
            && access_level_filter.satisfies(access_level)
    }
}
