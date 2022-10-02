use async_trait::async_trait;

use crate::data_source::DataSource;

/// Data source type which can save an item.
#[async_trait]
pub trait Save: DataSource {
    /// Saves the provided item into the storage.
    ///
    /// Returns the new instance of item as a result of saving provided item.
    async fn save(&self, item: Self::Item) -> Result<Self::Item, Self::Error>;
}
