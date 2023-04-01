//! Model of the workspace microservice domain layer.

pub use self::{
    id::{WorkspaceId, WorkspaceIdFilters},
    member::{MemberId, MemberIdFilters},
    name::{Name, NameError, NameFilters},
    workspace::{Workspace, WorkspaceData, WorkspaceDataFilters, WorkspaceFilters},
};

mod id;
mod member;
mod name;
mod workspace;
