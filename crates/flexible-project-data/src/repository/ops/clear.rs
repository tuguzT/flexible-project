use async_trait::async_trait;

use crate::repository::Repository;

/// Trait for repository which can remove all its data.
#[async_trait]
pub trait Clear: Repository {
    /// Clears this repository.
    ///
    /// Repository will contain no data of type [`Item`](Repository::Item)
    /// after performing this operation.
    async fn clear(&mut self);
}
