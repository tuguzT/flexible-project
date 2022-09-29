use async_trait::async_trait;

use crate::data_source::DataSource;

/// Data source type which can clear data source storage.
#[async_trait]
pub trait Clear: DataSource {
    /// The type returned when any error occurs.
    type Error;

    /// Clears storage of the data source.
    ///
    /// Data source will contain no data after performing this operation.
    async fn clear(&self) -> Result<(), Self::Error>;
}
