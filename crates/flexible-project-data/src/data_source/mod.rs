//! Definitions and utilities for data sources of the Flexible Project system.

use auto_impl::auto_impl;

pub use error::{Error, Result};

pub mod local;
pub mod user;

mod error;

/// Marker trait for data sources of the Flexible Project system.
///
/// It is used as the root trait for all the other data source traits
/// to share the same [`Item`](DataSource::Item) associated type for dependent traits.
#[auto_impl(&, Box, Arc)]
pub trait DataSource: Send + Sync {
    /// Type of item stored in this data source.
    type Item;
}
