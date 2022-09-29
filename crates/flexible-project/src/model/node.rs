#![allow(missing_docs)]

use async_graphql::{Interface, ID};

use crate::model::User;

/// Global Object Identification interface of the Flexible Project system.
#[derive(Interface)]
#[graphql(field(name = "id", type = "ID", desc = "The ID of the object."))]
pub enum Node {
    /// User variant of the interface object.
    User(User),
}
