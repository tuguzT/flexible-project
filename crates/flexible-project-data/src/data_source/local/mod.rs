//! Local implementations of data sources using MongoDB.

pub use error::LocalError;
pub use user::LocalUserDataSource;

mod error;
mod user;
mod utils;
