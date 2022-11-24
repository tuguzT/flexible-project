//! Definitions and utilities for data sources of the Flexible Project system.

use std::sync::Arc;

pub use error::{Error, Result};

pub mod local;
pub mod user;

mod error;

/// Marker trait for data sources of the Flexible Project system.
///
/// It is used as the root trait for all the other data source traits
/// to share the same [`Item`](DataSource::Item) associated type for dependent traits.
pub trait DataSource: Send + Sync {
    /// Type of item stored in this data source.
    type Item;
}

impl<T> DataSource for &T
where
    T: DataSource + ?Sized,
{
    type Item = T::Item;
}

impl<T> DataSource for Arc<T>
where
    T: DataSource + ?Sized,
{
    type Item = T::Item;
}
