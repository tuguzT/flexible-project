use fp_filter::GreaterEqual as DomainGreaterEqual;
use serde::{Deserialize, Serialize};

/// Serializable [greater equal filter](DomainGreaterEqual) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct GreaterEqual<T>(pub T);

impl<T> From<DomainGreaterEqual<T>> for GreaterEqual<T> {
    fn from(filter: DomainGreaterEqual<T>) -> Self {
        let DomainGreaterEqual(filter) = filter;
        Self(filter)
    }
}

impl<T> From<GreaterEqual<T>> for DomainGreaterEqual<T> {
    fn from(filter: GreaterEqual<T>) -> Self {
        let GreaterEqual(filter) = filter;
        Self(filter)
    }
}
