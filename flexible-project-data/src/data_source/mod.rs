//! Definitions and utilities for data sources of the Flexible Project system.

pub use crud::CrudDataSource;

use fp_core::model::Identifiable;

pub mod mock;
pub mod ops;
pub mod user;

mod crud;

/// Marker trait for data sources of the Flexible Project system.
///
/// It is used as the root trait for all the other data source traits
/// to share the same [`Item`](DataSource::Item) associated type for dependent traits.
pub trait DataSource {
    /// Type of item stored in this data source.
    type Item: Identifiable;
}
