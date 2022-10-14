#![allow(missing_docs)]

use crate::model::{ErasedId, Id, User};

use derive_more::{Display, From, IsVariant, Unwrap};

/// Types which could be identified by its [identifier](Id).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, From, IsVariant, Unwrap)]
pub enum Node {
    /// User variant of the node.
    User(User),
}

impl Node {
    /// Get an identifier of the node.
    pub fn id(&self) -> NodeId {
        match self {
            Node::User(user) => NodeId::User(user.id.clone()),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Display, From, IsVariant, Unwrap)]
pub enum NodeId {
    /// User variant of the identifier.
    User(Id<User>),
}

impl NodeId {
    /// Erases this node identifier explicitly,
    /// turning self into [`ErasedId`].
    pub fn erase(self) -> ErasedId {
        match self {
            NodeId::User(id) => id.erase(),
        }
    }
}
