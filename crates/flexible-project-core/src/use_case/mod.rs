//! Definitions and utilities for use cases of the Flexible Project system.
//!
//! *Use case* is a potential scenario in which a system receives an external request and responds to it.
//!
//! A use case object, or *interactor*, encapsulates and implements use cases of the system.

pub use hasher::{PasswordHashVerifier, PasswordHasher};
pub use id::IdGenerator;
pub use node::FindNode;
pub use user::{DeleteUser, FilterUsers, SignIn, SignUp, UpdateUser, UserTokenGenerator};
pub use verifier::{
    PasswordVerifier, UserCredentialsState, UserCredentialsVerifier, UserTokenVerifier,
    UsernameVerifier,
};

mod hasher;
mod id;
mod node;
mod user;
mod verifier;
