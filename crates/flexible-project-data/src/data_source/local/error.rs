use derive_more::{Display, Error, From};
use mongodb::error::Error as MongoError;

/// Local error of the local data source implementation.
#[derive(Debug, Display, Error, From)]
#[display(fmt = "local data source error: {}", _0)]
pub struct LocalError(#[error(source)] MongoError);
