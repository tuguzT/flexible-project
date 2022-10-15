use derive_more::{Display, Error, From};

use crate::data_source::Error as DataSourceError;

/// Result of repository operations with error type of [`struct@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of repository implementation.
#[derive(Debug, Display, Error, From)]
#[from(forward)]
pub struct Error(#[error(source)] ErrorKind);

#[derive(Debug, Display, Error, From)]
#[display(fmt = "repository error")]
#[from(forward)]
enum ErrorKind {
    DataSource(#[error(source)] DataSourceError),
}
