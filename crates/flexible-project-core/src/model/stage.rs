//! Project stage definitions and utilities for the Flexible Project system model.

use crate::model::id::Id;
use crate::model::task::Task;

/// Type of project [stage](Stage) identifier.
pub type StageId = Id<Stage>;

/// Stage of the Flexible Project system
/// is a collection of its own [tasks](Task).
///
/// Stage is a part of some [project](crate::model::project::Project).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stage {
    /// Identifier of the stage.
    pub id: Id<Self>,
    /// Name of the stage.
    pub name: String,
    /// Identifiers of [tasks](Task) of the stage.
    pub tasks: Vec<Id<Task>>,
}
