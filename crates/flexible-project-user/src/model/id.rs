use derive_more::Display;
use fp_core::id::ErasedId as CoreErasedId;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn};

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

/// Filters for identifier of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct ErasedIdFilters {
    /// Equality identifier filter.
    pub eq: Option<Equal<ErasedId>>,
    /// Inequality identifier filter.
    pub ne: Option<NotEqual<ErasedId>>,
    /// In identifier filter.
    pub r#in: Option<In<Vec<ErasedId>>>,
    /// Not in identifier filter.
    pub nin: Option<NotIn<Vec<ErasedId>>>,
}
