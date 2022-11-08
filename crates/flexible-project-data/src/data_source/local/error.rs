use derive_more::{Display, Error, From};
use mongodb::bson::ser::Error as SerError;
use mongodb::bson::uuid::Error as UuidError;
use mongodb::error::Error as MongoError;

/// Result of local data source operations with error type of [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Error of the local data source implementation.
#[derive(Debug, Display, Error, From)]
pub enum Error {
    Mongo(MongoError),
    Id(UuidError),
    Serialize(SerError),
}
