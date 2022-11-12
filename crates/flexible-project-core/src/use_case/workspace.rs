//! Workspace use cases of the Flexible Project system.

use async_trait::async_trait;
use derive_more::{Display, Error, From};

use crate::model::id::Id;
use crate::model::user::UserToken;
use crate::model::workspace::{Workspace, WorkspaceFilters};
use crate::use_case::error::InternalError;
use crate::use_case::user::CurrentUserError;

/// Error type of [create workspace](CreateWorkspace) use case.
#[derive(Debug, Display, From, Error)]
pub enum CreateWorkspaceError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// Provided name was already taken by another workspace.
    #[display(fmt = "name was already taken by another workspace")]
    AlreadyTaken,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can create new workspace.
#[async_trait]
pub trait CreateWorkspace {
    /// Creates new workspace with provided name.
    async fn create(
        &self,
        token: UserToken,
        name: String,
    ) -> Result<Workspace, CreateWorkspaceError>;
}

/// Interactor type which can filter all workspaces of the system.
#[async_trait]
pub trait FilterWorkspace {
    /// Filters all workspaces with provided [filters](WorkspaceFilters).
    ///
    /// Returns collection of filter results.
    async fn filter(&self, filters: WorkspaceFilters) -> Result<Vec<Workspace>, InternalError>;
}

// TODO update traits for workspace (add member, change owner, create project, etc.)

/// Error type of [delete workspace](DeleteWorkspace) use case.
#[derive(Debug, Display, From, Error)]
pub enum DeleteWorkspaceError {
    /// Current user error.
    CurrentUser(CurrentUserError),
    /// User does not allowed to delete workspace of another user.
    #[display(fmt = "not allowed to delete workspace of another user")]
    NotAllowed,
    /// No workspace was found by provided identifier.
    #[display(fmt = "no workspace to delete")]
    NoWorkspace,
    /// Use case internal error.
    Internal(InternalError),
}

/// Interactor type which can delete workspace from the system.
#[async_trait]
pub trait DeleteWorkspace {
    /// Deletes the workspace with provided identifier.
    ///
    /// Returns data of the deleted workspace if present.
    async fn delete(
        &self,
        token: UserToken,
        workspace_to_delete: Id<Workspace>,
    ) -> Result<Option<Workspace>, DeleteWorkspaceError>;
}
