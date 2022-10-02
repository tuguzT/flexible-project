use std::convert::Infallible;

use derive_more::{Display, Error, From};

use crate::data_source::local::LocalError;

/// General error type of the library.
#[derive(Debug, Display, Error, From)]
#[from(forward)]
pub struct Error(ErrorKind);

#[derive(Debug, Display, Error, From)]
#[from(forward)]
enum ErrorKind {
    Local(#[error(source)] LocalError),
}

impl From<Infallible> for ErrorKind {
    fn from(never: Infallible) -> Self {
        match never {}
    }
}
