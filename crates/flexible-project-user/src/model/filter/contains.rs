use fp_filter::Contains as DomainContains;
use serde::{Deserialize, Serialize};

/// Serializable [contains filter](DomainContains) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Contains<T>(pub T);

impl<T> From<DomainContains<T>> for Contains<T> {
    fn from(filter: DomainContains<T>) -> Self {
        let DomainContains(filter) = filter;
        Self(filter)
    }
}

impl<T> From<Contains<T>> for DomainContains<T> {
    fn from(filter: Contains<T>) -> Self {
        let Contains(filter) = filter;
        Self(filter)
    }
}
