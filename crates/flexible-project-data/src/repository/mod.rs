//! Definitions and utilities for repositories of the Flexible Project system.

pub use crud::CrudRepository;

pub mod ops;
pub mod user;

mod crud;

/// Marker trait for repositories of the Flexible Project system.
///
/// It is used as the root trait for all the other repository traits
/// to share the same [`Item`](Repository::Item) associated type for dependent traits.
pub trait Repository {
    /// Type of item stored in this repository.
    type Item;

    /// Type returned when any error occurs.
    type Error;
}
