//! Local implementations of data sources using MongoDB.

pub use client::Client;
pub(crate) use error::Error;
pub use user::LocalUserDataSource;

mod client;
mod error;
mod model;
mod user;
mod utils;
