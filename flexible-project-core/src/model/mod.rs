//! Module contains models of data
//! which is stored in the Flexible Project system.

pub use id::{Id, Identifiable};
pub use user::{User, UserRole, UserCredentials};

mod id;
mod user;
