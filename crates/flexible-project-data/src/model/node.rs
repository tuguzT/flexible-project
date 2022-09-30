#![allow(missing_docs)]

use derive_more::{From, IsVariant, Unwrap};
use fp_core::model::Node as CoreNode;
use serde::{Deserialize, Serialize};

use crate::model::{Id, User};

/// Enum that combines all variants of the node of the Flexible Project system.
#[derive(
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    From,
    IsVariant,
    Unwrap,
    Serialize,
    Deserialize,
)]
pub enum Node {
    /// User variant of the node.
    User(User),
}

impl CoreNode for Node {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        match self {
            Node::User(user) => user
                .id()
                .to_string()
                .parse()
                .expect("string from another ID is always valid ID"),
        }
    }
}
