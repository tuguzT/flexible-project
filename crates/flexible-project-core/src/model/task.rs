//! Task definitions and utilities for the Flexible Project system model.

use crate::model::id::Id;

/// Type of stage [task](Task) identifier.
pub type TaskId = Id<Task>;

/// Task of the Flexible Project system
/// is a unit of work to be completed
/// by the [project](crate::model::project::Project) team.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    /// Identifier of the task.
    pub id: Id<Self>,
    /// Name of the task.
    pub name: String,
}
