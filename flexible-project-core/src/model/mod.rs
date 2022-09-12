//! Models of data stored in the Flexible Project system.

pub use id::{Id, Identifiable};
pub use user::{User, UserCredentials, UserRole};

mod id;
mod user;
