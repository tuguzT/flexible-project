use async_trait::async_trait;

use crate::repository::Repository;

/// Repository type which can remove all its data.
#[async_trait]
pub trait Clear: Repository {
    /// The type returned when any error occurs.
    type Error;

    /// Clears this repository.
    ///
    /// Repository will contain no data after performing this operation.
    async fn clear(&self) -> Result<(), Self::Error>;
}
