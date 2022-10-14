//! Definitions of GraphQL objects of the Flexible Project system.

pub use node::Node;
pub use user::{UpdateUser, User, UserCredentials, UserRole};

mod node;
mod user;
