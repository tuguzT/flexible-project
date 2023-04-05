use std::borrow::Borrow;

use fp_filter::{Equal, Filter, In, NotEqual, NotIn};
use typed_builder::TypedBuilder;

use crate::model::{MemberId, ProjectId, RoleName};

use super::RoleUpdateOperationScope as Scope;

/// Operation of update role access level which can modify different workspace aspects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoleUpdateOperation {
    /// Member can update general information of the workspace,
    /// such as name and description.
    UpdateWorkspace,
    /// Member can create new project in the workspace.
    CreateProject,
    /// Member can delete existing project of the workspace.
    DeleteProject(Scope<ProjectId>),
    /// Member can add another user (as a new member) into the workspace.
    AddMember,
    /// Member can remove another member from the workspace.
    RemoveMember(Scope<MemberId>),
    /// Member can create new role in the workspace.
    CreateRole,
    /// Member can update data of existing role in the workspace,
    /// such as name and access level.
    UpdateRole(Scope<RoleName>),
    /// Member can delete existing role in the workspace.
    DeleteRole(Scope<RoleName>),
    /// Member can grant an existing role to another member of the workspace.
    GrantRole(Scope<MemberId>),
    /// Member can revoke an existing role from another member of the workspace.
    RevokeRole(Scope<MemberId>),
}

/// Filters for workspace role update operation of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleUpdateOperationFilters<'a> {
    /// Equality workspace role update operation filter.
    pub eq: Option<Equal<&'a RoleUpdateOperation>>,
    /// Inequality workspace role update operation filter.
    pub ne: Option<NotEqual<&'a RoleUpdateOperation>>,
    /// In workspace role update operation filter.
    pub r#in: Option<In<&'a [RoleUpdateOperation]>>,
    /// Not in workspace role update operation filter.
    pub nin: Option<NotIn<&'a [RoleUpdateOperation]>>,
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
