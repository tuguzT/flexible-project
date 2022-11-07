//! Node utilities for the Flexible Project server model.

#![allow(missing_docs)]

use async_graphql::{Interface, ID};
use derive_more::{IsVariant, Unwrap};
use fp_core::model::node::Node as CoreNode;

use crate::model::user::User;

/// Global Object Identification interface of the Flexible Project system.
#[derive(Interface, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, IsVariant, Unwrap)]
#[graphql(field(name = "id", type = "ID", desc = "The ID of the object."))]
pub enum Node {
    /// User variant of the interface object.
    User(User),
}

impl From<Node> for CoreNode {
    fn from(node: Node) -> Self {
        match node {
            Node::User(user) => CoreNode::User(user.into()),
        }
    }
}

impl From<CoreNode> for Node {
    fn from(node: CoreNode) -> Self {
        match node {
            CoreNode::User(user) => Node::User(user.into()),
            CoreNode::Workspace(_) => todo!(),
            CoreNode::Project(_) => todo!(),
            CoreNode::Stage(_) => todo!(),
            CoreNode::Task(_) => todo!(),
        }
    }
}
