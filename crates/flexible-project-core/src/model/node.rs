//! Node definitions and utilities for the Flexible Project system model.

#![allow(missing_docs)]

use crate::model::id::{ErasedId, Id};
use crate::model::project::Project;
use crate::model::stage::Stage;
use crate::model::task::Task;
use crate::model::user::User;
use crate::model::workspace::Workspace;

use derive_more::{Display, From, IsVariant, Unwrap};

/// Types which could be identified by its [identifier](Id).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, From, IsVariant, Unwrap)]
pub enum Node {
    /// [User] variant of the node.
    User(User),
    /// [Workspace] variant of the node.
    Workspace(Workspace),
    /// [Project] variant of the node.
    Project(Project),
    /// [Stage] variant of the node.
    Stage(Stage),
    /// [Task] variant of the node.
    Task(Task),
}

impl Node {
    /// Get an [identifier](NodeId) of the node.
    ///
    /// Note that this will clone underlying identifier.
    pub fn id(&self) -> NodeId {
        match self {
            Node::User(user) => NodeId::User(user.id.clone()),
            Node::Workspace(workspace) => NodeId::Workspace(workspace.id.clone()),
            Node::Project(project) => NodeId::Project(project.id.clone()),
            Node::Stage(stage) => NodeId::Stage(stage.id.clone()),
            Node::Task(task) => NodeId::Task(task.id.clone()),
        }
    }

    /// Converts this node into the node [identifier](NodeId).
    pub fn into_id(self) -> NodeId {
        self.into()
    }
}

impl From<Node> for NodeId {
    fn from(node: Node) -> Self {
        match node {
            Node::User(user) => NodeId::User(user.id),
            Node::Workspace(workspace) => NodeId::Workspace(workspace.id),
            Node::Project(project) => NodeId::Project(project.id),
            Node::Stage(stage) => NodeId::Stage(stage.id),
            Node::Task(task) => NodeId::Task(task.id),
        }
    }
}

/// Identifier type of the [`Node`].
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Display, From, IsVariant, Unwrap)]
pub enum NodeId {
    /// [User] variant of the node identifier.
    User(Id<User>),
    /// [Workspace] variant of the node identifier.
    Workspace(Id<Workspace>),
    /// [Project] variant of the node identifier.
    Project(Id<Project>),
    /// [Stage] variant of the node identifier.
    Stage(Id<Stage>),
    /// [Task] variant of the node identifier.
    Task(Id<Task>),
}

impl NodeId {
    /// Erases this node identifier explicitly,
    /// turning self into [`ErasedId`].
    pub fn erase(self) -> ErasedId {
        match self {
            NodeId::User(id) => id.erase(),
            NodeId::Workspace(id) => id.erase(),
            NodeId::Project(id) => id.erase(),
            NodeId::Stage(id) => id.erase(),
            NodeId::Task(id) => id.erase(),
        }
    }
}

impl From<NodeId> for String {
    fn from(id: NodeId) -> Self {
        id.erase().into()
    }
}
