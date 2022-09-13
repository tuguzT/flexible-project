use async_trait::async_trait;

use crate::repository::Repository;

/// Trait for repository which can save an item.
#[async_trait]
pub trait Save: Repository {
    /// Saves the provided item.
    ///
    /// Returns the new instance of item as a result of saving provided item.
    async fn save(&mut self, item: Self::Item) -> Self::Item;
}
