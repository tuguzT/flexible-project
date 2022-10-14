use derive_more::{Display, Error, From};

use crate::data_source::local::Error as LocalError;

/// Result of data source operations with error type of [`struct@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of some data source implementation.
#[derive(Debug, Display, Error, From, Clone)]
#[from(forward)]
pub struct Error(#[error(source)] ErrorKind);

#[derive(Debug, Display, Error, From, Clone)]
#[display(fmt = "data source error")]
#[from(forward)]
enum ErrorKind {
    Local(#[error(source)] LocalError),
}
