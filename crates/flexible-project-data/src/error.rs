use std::convert::Infallible;

use derive_more::{Display, Error, From};

use crate::interactor::Error as InteractorError;

/// General result type of the library.
pub type Result<T> = std::result::Result<T, Error>;

/// General error type of the library.
#[derive(Debug, Display, Error, From)]
#[from(forward)]
pub struct Error(#[error(source)] ErrorKind);

#[derive(Debug, Display, Error, From)]
#[from(forward)]
enum ErrorKind {
    Interactor(#[error(source)] InteractorError),
}

impl From<Infallible> for ErrorKind {
    fn from(never: Infallible) -> Self {
        match never {}
    }
}
