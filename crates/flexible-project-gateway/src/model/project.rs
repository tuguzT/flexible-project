//! Project data model of the gateway service.

use async_graphql::{SimpleObject, ID};

/// Project properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {
    /// Unique identifier of the project.
    pub id: ID,
}
