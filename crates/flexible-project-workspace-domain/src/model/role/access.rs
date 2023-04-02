use std::borrow::Borrow;

use derive_more::Display;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn};
use indexmap::IndexSet;
use typed_builder::TypedBuilder;

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
    pub eq: Option<Equal<'a, RoleAccessLevel>>,
    /// Inequality workspace role role access level filter.
    pub ne: Option<NotEqual<'a, RoleAccessLevel>>,
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

/// Operation of update role access level which can modify different workspace aspects.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoleUpdateOperation {
    /// Member can add a member into the workspace.
    AddMember,
    /// Member can remove a member from the workspace.
    RemoveMember,
    /// Member can create new project in the workspace.
    CreateProject,
    /// Member can delete existing project of the workspace.
    DeleteProject,
}

/// Filters for workspace role update operation of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleUpdateOperationFilters<'a> {
    /// Equality workspace role update operation filter.
    pub eq: Option<Equal<'a, RoleUpdateOperation>>,
    /// Inequality workspace role update operation filter.
    pub ne: Option<NotEqual<'a, RoleUpdateOperation>>,
    /// In workspace role update operation filter.
    pub r#in: Option<In<'a, RoleUpdateOperation>>,
    /// Not in workspace role update operation filter.
    pub nin: Option<NotIn<'a, RoleUpdateOperation>>,
}

impl<Input> Filter<Input> for RoleUpdateOperationFilters<'_>
where
    Input: Borrow<RoleUpdateOperation>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { eq, ne, r#in, nin } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}
