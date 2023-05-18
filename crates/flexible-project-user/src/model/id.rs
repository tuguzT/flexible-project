use derive_more::Display;
use fp_core::id::ErasedId as CoreErasedId;
use serde::{Deserialize, Serialize};

/// Serializable [erased identifier](CoreErasedId) of the system.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ErasedId(String);

impl From<CoreErasedId> for ErasedId {
    fn from(id: CoreErasedId) -> Self {
        let id = id.into_inner();
        Self(id)
    }
}

impl From<ErasedId> for CoreErasedId {
    fn from(id: ErasedId) -> Self {
        let ErasedId(id) = id;
        CoreErasedId::new(id)
    }
}
