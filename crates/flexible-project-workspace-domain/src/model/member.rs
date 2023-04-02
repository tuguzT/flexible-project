use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_filter::Filter;
use fp_user_domain::model::{UserId, UserIdFilters};
use typed_builder::TypedBuilder;

use super::{role::RoleName, RoleNameFilters};

/// Member of the workspace in the system.
#[derive(Debug, Clone)]
pub struct Member {
    /// Identifier of the user which is a member of the workspace.
    pub user_id: UserId,
    /// Name of role of the workspace.
    pub role_name: RoleName,
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for Member {}

impl Hash for Member {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
    }
}

/// Filters for workspace member of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct MemberFilters<'a> {
    /// Member user identifier filters.
    pub user_id: Option<UserIdFilters<'a>>,
    /// Member role name filters.
    pub role_name: Option<RoleNameFilters<'a>>,
}

impl<Input> Filter<Input> for MemberFilters<'_>
where
    Input: Borrow<Member>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            user_id: user_id_filter,
            role_name: role_name_filter,
        } = self;
        let Member { user_id, role_name } = input.borrow();
        user_id_filter.satisfies(user_id) && role_name_filter.satisfies(role_name)
    }
}
