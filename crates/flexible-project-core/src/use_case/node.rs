//! Node use cases of the Flexible Project system.

use async_trait::async_trait;
use auto_impl::auto_impl;

use crate::model::{id::ErasedId, node::Node};

use super::error::InternalError;

/// Interactor type which can find any node of the system by its identifier.
#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait FindNode: Send + Sync {
    /// Returns [`Some`] if node by provided identifier exists, [`None`] otherwise.
    async fn find(&self, id: ErasedId) -> Result<Option<Node>, InternalError>;
}
