//! Definitions and utilities for use cases of the Flexible Project system.
//!
//! *Use case* is a potential scenario in which a system receives an external request and responds to it.
//!
//! A use case object, or *interactor*, encapsulates and implements use cases of the system.

pub use hasher::{PasswordHashVerifier, PasswordHasher};
pub use node::FindNode;
pub use user::{CreateUser, DeleteUser, FilterUsers, UpdateUser};
pub use verifier::{PasswordVerifier, UserCredentialsVerifier, UsernameVerifier};

mod hasher;
mod node;
mod user;
mod verifier;
