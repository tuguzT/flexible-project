#![allow(missing_docs)]

use async_graphql::{Interface, ID};
use derive_more::{IsVariant, Unwrap};
use fp_core::model::Node as CoreNode;
use fp_data::model::{Id, Node as DataNode};

use crate::model::User;

/// Global Object Identification interface of the Flexible Project system.
#[derive(Interface, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, IsVariant, Unwrap)]
#[graphql(field(name = "id", type = "ID", desc = "The ID of the object."))]
pub enum Node {
    /// User variant of the interface object.
    User(User),
}

impl From<Node> for DataNode {
    fn from(node: Node) -> Self {
        match node {
            Node::User(user) => DataNode::User(user.into()),
        }
    }
}

impl From<DataNode> for Node {
    fn from(node: DataNode) -> Self {
        match node {
            DataNode::User(user) => Node::User(user.into()),
        }
    }
}

impl CoreNode for Node {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        match self {
            Self::User(user) => CoreNode::id(user)
                .to_string()
                .parse()
                .expect("string from another ID is always valid ID"),
        }
    }
}
