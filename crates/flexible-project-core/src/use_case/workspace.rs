//! Workspace use cases of the Flexible Project system.

use async_trait::async_trait;

use crate::model::id::Id;
use crate::model::user::User;
use crate::model::workspace::{Workspace, WorkspaceFilters};

/// Input object which contains data needed to create new workspace.
pub struct CreateWorkspaceInput {
    /// Creator of the new workspace.
    pub creator: Id<User>,
    /// Name of the new workspace.
    pub name: String,
}

/// Interactor type which can create new workspace.
#[async_trait]
pub trait CreateWorkspace {
    /// The type returned when any error occurs.
    type Error;

    /// Creates new workspace from provided [input](CreateWorkspaceInput)
    /// in the Flexible Project system.
    async fn create(&self, input: CreateWorkspaceInput) -> Result<Workspace, Self::Error>;
}

/// Interactor type which can filter all workspaces of the system.
#[async_trait]
pub trait FilterWorkspace {
    /// The type returned when any error occurs.
    type Error;

    /// Filters all workspaces with provided [filters](WorkspaceFilters).
    ///
    /// Returns collection of filter results.
    async fn filter(&self, filters: WorkspaceFilters) -> Result<Vec<Workspace>, Self::Error>;
}

// TODO update traits for workspace (add member, change owner, create project, etc.)

/// Interactor type which can delete workspace from the system.
#[async_trait]
pub trait DeleteWorkspace {
    /// The type returned when any error occurs.
    type Error;

    /// Deletes the workspace with provided identifier.
    ///
    /// Returns data of the deleted workspace if present.
    async fn delete(&self, id: Id<Workspace>) -> Result<Option<Workspace>, Self::Error>;
}
