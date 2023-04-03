use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_filter::Filter;
use indexmap::IndexSet;
use typed_builder::TypedBuilder;

use super::{
    description::{Description, DescriptionFilters},
    id::{WorkspaceId, WorkspaceIdFilters},
    member::{Member, MemberFilters},
    name::{Name, NameFilters},
    role::{Role, RoleFilters},
    visibility::{Visibility, VisibilityFilters},
};

/// Model of workspace in the system.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Unique identifier of the workspace.
    pub id: WorkspaceId,
    /// Data of the workspace.
    pub data: WorkspaceData,
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Workspace {}

impl Hash for Workspace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Set of roles of the workspace.
pub type Roles = IndexSet<Role>;

/// Set of members of the workspace.
pub type Members = IndexSet<Member>;

/// Data of the workspace in the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceData {
    /// Name of the workspace.
    pub name: Name,
    /// Description of the workspace.
    pub description: Description,
    /// Visibility of the workspace.
    pub visibility: Visibility,
    /// Roles of the workspace.
    pub roles: Roles,
    /// Members of the workspace.
    pub members: Members,
}

/// Filters for workspaces of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct WorkspaceFilters<'a> {
    /// Workspace identifier filters.
    pub id: Option<WorkspaceIdFilters<'a>>,
    /// Workspace data filters.
    pub data: Option<WorkspaceDataFilters<'a>>,
}

impl<Input> Filter<Input> for WorkspaceFilters<'_>
where
    Input: Borrow<Workspace>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            id: id_filter,
            data: data_filter,
        } = self;
        let Workspace { id, data } = input.borrow();
        id_filter.satisfies(id) && data_filter.satisfies(data)
    }
}

/// Filters for workspace data of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct WorkspaceDataFilters<'a> {
    /// Workspace name filters.
    pub name: Option<NameFilters<'a>>,
    /// Workspace description filters.
    pub description: Option<DescriptionFilters<'a>>,
    /// Workspace visibility filters.
    pub visibility: Option<VisibilityFilters<'a>>,
    /// Workspace roles filters.
    pub roles: Option<RolesFilters<'a>>,
    /// Workspace members filters.
    pub members: Option<MembersFilters<'a>>,
}

impl<Input> Filter<Input> for WorkspaceDataFilters<'_>
where
    Input: Borrow<WorkspaceData>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            name: name_filter,
            description: description_filter,
            visibility: visibility_filter,
            roles: roles_filter,
            members: members_filter,
        } = self;
        let WorkspaceData {
            name,
            description,
            visibility,
            roles,
            members,
        } = input.borrow();
        name_filter.satisfies(name)
            && visibility_filter.satisfies(visibility)
            && description_filter.satisfies(description)
            && roles_filter.satisfies(roles)
            && members_filter.satisfies(members)
    }
}

/// Filters for set of workspace roles of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RolesFilters<'a> {
    /// If input contains roles which satisfies role filters.
    pub contains: Option<RoleFilters<'a>>,
    /// If input does not contain roles which satisfies role filters.
    pub not_contains: Option<RoleFilters<'a>>,
}

impl<Input> Filter<Input> for RolesFilters<'_>
where
    Input: IntoIterator,
    <Input as IntoIterator>::IntoIter: Clone,
    <Input as IntoIterator>::Item: Borrow<Role>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            contains: contains_filter,
            not_contains: not_contains_filter,
        } = self;
        let mut contains = input.into_iter();
        let mut not_contains = contains.clone();
        contains.any(|item| contains_filter.satisfies(item))
            && !not_contains.any(|item| not_contains_filter.satisfies(item))
    }
}

/// Filters for set of workspace members of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct MembersFilters<'a> {
    /// If input contains members which satisfies member filters.
    pub contains: Option<MemberFilters<'a>>,
    /// If input does not contain members which satisfies member filters.
    pub not_contains: Option<MemberFilters<'a>>,
}

impl<Input> Filter<Input> for MembersFilters<'_>
where
    Input: IntoIterator,
    <Input as IntoIterator>::IntoIter: Clone,
    <Input as IntoIterator>::Item: Borrow<Member>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            contains: contains_filter,
            not_contains: not_contains_filter,
        } = self;
        let mut contains = input.into_iter();
        let mut not_contains = contains.clone();
        contains.any(|item| contains_filter.satisfies(item))
            && !not_contains.any(|item| not_contains_filter.satisfies(item))
    }
}
