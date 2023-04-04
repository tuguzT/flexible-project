//! Definitions and utilities for objects which have access to the outer environment.

pub use self::{id::GenerateUserId, user::UserDatabase};

mod id;
mod user;
