use derive_more::{Display, Error, From};

use crate::data_source::Error as DataSourceError;

/// Result of repository operations with error type of [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of repository implementation.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    /// Data source error variant.
    DataSource(DataSourceError),
}
