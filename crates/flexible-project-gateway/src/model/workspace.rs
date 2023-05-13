//! Workspace data model of the gateway service.

use async_graphql::{Enum, InputObject, Object, SimpleObject, ID};

/// Query object of workspaces of the Flexible Project system.
#[derive(Debug, Default)]
pub struct WorkspaceQuery;

#[Object]
impl WorkspaceQuery {
    /// Filters all workspaces of the system.
    pub async fn workspaces(&self, filters: WorkspaceFilters) -> Vec<Workspace> {
        let _ = filters;
        None.unwrap()
    }

    /// Creates new workspace with provided name in the system.
    /// Newly created workspace will be owned by the user that created it.
    pub async fn create_workspace(&self, user_id: ID, name: String) -> Workspace {
        let _ = (user_id, name);
        None.unwrap()
    }

    /// Update properties of the workspace by provided identifier with provided data.
    pub async fn update_workspace(&self, id: ID, update: UpdateWorkspace) -> Workspace {
        let _ = (id, update);
        None.unwrap()
    }

    /// Deletes workspace from the system by provided identifier.
    pub async fn delete_workspace(&self, id: ID) -> Workspace {
        let _ = id;
        None.unwrap()
    }
}

/// Workspace properties of the Flexible project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workspace {
    /// Unique identifier of the workspace.
    pub id: ID,
    /// Name of the workspace.
    pub name: String,
    /// Description of the workspace.
    pub description: String,
    /// Visibility of the workspace.
    pub visibility: WorkspaceVisibility,
    /// Optional image of the workspace.
    pub image_url: Option<String>,
    /// Members of the workspace.
    pub members: Vec<WorkspaceMember>,
    /// Roles of the workspace.
    pub roles: Vec<WorkspaceRole>,
}

/// Filters of workspaces of the Flexible Project system.
#[derive(Debug, InputObject)]
pub struct WorkspaceFilters {
    /// Identifier filter of the workspace.
    pub id: Option<ID>,
}

/// Data of the workspace to update.
#[derive(Debug, InputObject)]
pub struct UpdateWorkspace {
    /// Name of the workspace to update, if present.
    pub name: Option<String>,
    /// Description of the workspace to update, if present.
    pub description: Option<String>,
    /// Visibility of the workspace to update, if present.
    pub visibility: Option<WorkspaceVisibility>,
    /// Optional image of the workspace to update, if present.
    pub image_url: Option<Option<String>>,
}

/// Visibility level of the workspace of the Flexible Project system.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkspaceVisibility {
    /// Workspace is visible for any user of the system.
    Public,
    /// Workspace is only visible for members of this workspace.
    Private,
}

/// Workspace member properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceMember {
    /// Identifier of user which is a member of the workspace.
    pub user_id: ID,
    /// Identifier of role of the workspace.
    pub role_id: ID,
}

/// Workspace role properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceRole {
    /// Unique identifier of the role.
    pub id: ID,
    /// Unique name of the role in the workspace.
    pub name: String,
    /// Set of available operations which modify workspace data.
    pub operations: Vec<WorkspaceUpdateOperation>,
}

/// Operation of update role access level which can modify different workspace aspects.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkspaceUpdateOperation {
    /// Member can update general information of the workspace,
    /// such as name and description.
    UpdateWorkspace,
    /// Member can create new project in the workspace.
    CreateProject,
    /// Member can delete existing project of the workspace.
    DeleteProject, // TODO (Scope<ProjectId>)
    /// Member can add another user (as a new member) into the workspace.
    AddMember,
    /// Member can remove another member from the workspace.
    RemoveMember, // TODO (Scope<MemberId>)
    /// Member can create new role in the workspace.
    CreateRole,
    /// Member can update data of existing role in the workspace,
    /// such as name and access level.
    UpdateRole, // TODO (Scope<RoleId>)
    /// Member can delete existing role in the workspace.
    DeleteRole, // TODO (Scope<RoleId>)
    /// Member can grant an existing role to another member of the workspace.
    GrantRole, // TODO (Scope<MemberId>)
    /// Member can revoke an existing role from another member of the workspace.
    RevokeRole, // TODO (Scope<MemberId>)
}
