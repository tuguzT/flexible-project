//! Model of the workspace microservice domain layer.

pub use self::{
    description::{Description, DescriptionFilters},
    id::{WorkspaceId, WorkspaceIdFilters},
    member::*,
    name::{Name, NameError, NameFilters},
    role::*,
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
