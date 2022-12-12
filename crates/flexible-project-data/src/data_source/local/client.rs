use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;

use crate::data_source::local::Error;
use crate::data_source::Result;

/// Type wrapper around [MongoDB client](MongoClient).
#[derive(Debug, Clone)]
pub struct Client(pub(super) MongoClient);

impl Client {
    /// Creates new client instance.
    pub async fn new(conn_str: impl AsRef<str>) -> Result<Self> {
        let mut client_options = ClientOptions::parse(conn_str).await.map_err(Error::from)?;
        client_options.app_name = Some("flexible-project".into());

        let client = MongoClient::with_options(client_options).map_err(Error::from)?;
        Ok(Self(client))
    }
}
