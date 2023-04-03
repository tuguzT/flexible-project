//! Model of the workspace microservice domain layer.

pub use self::{
    description::{Description, DescriptionFilters},
    id::{WorkspaceId, WorkspaceIdFilters},
    member::{Member, MemberFilters},
    name::{Name, NameError, NameFilters},
    role::{
        Role, RoleAccessLevel, RoleAccessLevelFilters, RoleFilters, RoleName, RoleNameError,
        RoleNameFilters, RoleUpdateOperation, RoleUpdateOperationFilters, RoleUpdateOperations,
    },
    visibility::{Visibility, VisibilityFilters},
    workspace::{
        Members, MembersFilters, Roles, RolesFilters, Workspace, WorkspaceData,
        WorkspaceDataFilters, WorkspaceFilters,
    },
};

mod description;
mod id;
mod member;
mod name;
mod role;
mod visibility;
mod workspace;
