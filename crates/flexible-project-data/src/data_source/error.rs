use derive_more::{Display, Error, From};

use crate::data_source::local::Error as LocalError;

/// Result of data source operations with error type of [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of some data source implementation.
#[derive(Debug, Display, Error, From)]
#[from(forward)]
pub enum Error {
    /// Local error variant.
    Local(#[error(source)] LocalError),
}
