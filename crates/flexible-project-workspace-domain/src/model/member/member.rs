#![allow(clippy::module_inception)]

use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_filter::Filter;
use typed_builder::TypedBuilder;

use super::{
    super::role::{RoleName, RoleNameFilters},
    MemberId, MemberIdFilters,
};

/// Member of the workspace in the system.
#[derive(Debug, Clone)]
pub struct Member {
    /// Identifier of member of the workspace.
    pub id: MemberId,
    /// Name of role of the workspace.
    pub role_name: RoleName,
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Member {}

impl Hash for Member {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Filters for workspace member of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct MemberFilters<'a> {
    /// Member identifier filters.
    pub id: Option<MemberIdFilters<'a>>,
    /// Member role name filters.
    pub role_name: Option<RoleNameFilters<'a>>,
}

impl<Input> Filter<Input> for MemberFilters<'_>
where
    Input: Borrow<Member>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            id: id_filter,
            role_name: role_name_filter,
        } = self;
        let Member { id, role_name } = input.borrow();
        id_filter.satisfies(id) && role_name_filter.satisfies(role_name)
    }
}
