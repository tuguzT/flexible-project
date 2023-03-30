//! Implementation of local user repository.

pub use self::{
    id::LocalIdGenerator,
    user::{LocalError, LocalUserDatabase, LocalUsers},
};

mod filter;
mod id;
mod user;
