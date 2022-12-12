//! Definitions and utilities for repositories of the Flexible Project system.

use auto_impl::auto_impl;

pub use error::{Error, Result};

pub mod user;

mod error;

/// Marker trait for repositories of the Flexible Project system.
///
/// It is used as the root trait for all the other repository traits
/// to share the same [`Item`](Repository::Item) associated type for dependent traits.
#[auto_impl(&, Box, Arc)]
pub trait Repository: Send + Sync {
    /// Type of item stored in this repository.
    type Item;
}
