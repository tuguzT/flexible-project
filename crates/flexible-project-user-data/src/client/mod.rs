//! Implementation of local repository client.

use derive_more::{Display, Error, From};
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client as DatabaseClient;

/// Local repository client.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) inner: DatabaseClient,
}

impl Client {
    /// Creates new client instance.
    pub async fn new(conn_str: impl AsRef<str>) -> Result<Self, ClientError> {
        let mut client_options = ClientOptions::parse(conn_str).await?;
        client_options.app_name = Some("flexible-project".into());

        let inner = DatabaseClient::with_options(client_options)?;
        Ok(Self { inner })
    }
}

/// Error which is returned when client creation failure occurred.
#[derive(Debug, Display, Clone, From, Error)]
pub struct ClientError {
    inner: Error,
}
