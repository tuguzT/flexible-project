//! Project definitions and utilities for the Flexible Project system model.

use crate::model::id::Id;
use crate::model::stage::Stage;
use crate::model::user::User;

/// Type of [project](Project) identifier.
pub type ProjectId = Id<Project>;

/// Project of the Flexible Project system
/// is a collection of its own [stages](Stage)
/// with [users](User) which have access to this project.
///
/// Project is a part of some [workspace](crate::model::workspace::Workspace).
pub struct Project {
    /// Identifier of the project.
    pub id: Id<Self>,
    /// Name of the project.
    pub name: String,
    /// Identifiers of [members](User) of the project.
    pub users: Vec<Id<User>>,
    /// Identifiers of [stages](Stage) of the project.
    pub stages: Vec<Id<Stage>>,
}
