use async_trait::async_trait;

use crate::repository::Repository;

/// Trait for repository which can save an item.
#[async_trait]
pub trait Save: Repository {
    /// The type returned when any error occurs.
    type Error;

    /// Saves the provided item.
    ///
    /// Returns the new instance of item as a result of saving provided item.
    async fn save(&self, item: Self::Item) -> Result<Self::Item, Self::Error>;
}
