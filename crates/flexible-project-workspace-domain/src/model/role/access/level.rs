use std::borrow::Borrow;

use fp_filter::{Equal, Filter, NotEqual};
use indexmap::IndexSet;
use typed_builder::TypedBuilder;

use super::operation::RoleUpdateOperation;

/// Set of available operations which modify workspace data.
pub type RoleUpdateOperations = IndexSet<RoleUpdateOperation>;

/// Access level of the workspace role to the workspace data.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RoleAccessLevel {
    /// Member has only read access to workspace data.
    #[default]
    Read,
    /// Member can read and modify different workspace aspects.
    Update(RoleUpdateOperations),
}

/// Filters for workspace role access level of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleAccessLevelFilters<'a> {
    /// Equality workspace role role access level filter.
    pub eq: Option<Equal<&'a RoleAccessLevel>>,
    /// Inequality workspace role role access level filter.
    pub ne: Option<NotEqual<&'a RoleAccessLevel>>,
}

impl<Input> Filter<Input> for RoleAccessLevelFilters<'_>
where
    Input: Borrow<RoleAccessLevel>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { eq, ne } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input)
    }
}
