use derive_more::{Display, Error, From};

use crate::data_source::Error as DataSourceError;
use crate::interactor::Error as InteractorError;

/// General result type of the library.
pub type Result<T> = std::result::Result<T, Error>;

/// General error type of the library.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    /// Interactor error variant.
    Interactor(#[error(source)] InteractorError),
    /// Data source error variant.
    DataSource(#[error(source)] DataSourceError),
}
