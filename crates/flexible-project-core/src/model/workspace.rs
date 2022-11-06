//! Workspace definitions and utilities for the Flexible Project system model.

use crate::model::id::Id;
use crate::model::project::Project;
use crate::model::user::User;

/// Type of [workspace](Workspace) identifier.
pub type WorkspaceId = Id<Workspace>;

/// Workspace of the Flexible Project system
/// is a collection of [projects](Project)
/// with [users](User) which have access to these projects.
pub struct Workspace {
    /// Identifier of the workspace.
    pub id: Id<Self>,
    /// Name of the workspace.
    pub name: String,
    /// Identifiers of [members](User) of the workspace.
    pub users: Vec<Id<User>>,
    /// Identifiers of [projects](Project) of the workspace.
    pub projects: Vec<Id<Project>>,
}
