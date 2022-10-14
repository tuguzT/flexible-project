use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;

use crate::data_source::Result;

/// Type wrapper around [MongoDB client](MongoClient).
pub struct Client(pub MongoClient);

impl Client {
    /// Creates new client instance.
    pub fn new() -> Result<Self> {
        let client_options = ClientOptions::builder()
            .app_name("flexible-project".to_string())
            .build();
        let client = MongoClient::with_options(client_options)?;
        Ok(Self(client))
    }
}
