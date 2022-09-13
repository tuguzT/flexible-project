use async_trait::async_trait;

use crate::data_source::DataSource;

/// Trait for data source which can clear its storage.
#[async_trait]
pub trait Clear: DataSource {
    /// Clears storage of the data source.
    ///
    /// Data source will contain no data of type [`Item`](DataSource::Item)
    /// after performing this operation.
    async fn clear(&mut self);
}