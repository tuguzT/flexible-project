use fp_filter::GreaterThan as DomainGreaterThan;
use serde::{Deserialize, Serialize};

/// Serializable [greater than filter](DomainGreaterThan) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct GreaterThan<T>(pub T);

impl<T> From<DomainGreaterThan<T>> for GreaterThan<T> {
    fn from(filter: DomainGreaterThan<T>) -> Self {
        let DomainGreaterThan(filter) = filter;
        Self(filter)
    }
}

impl<T> From<GreaterThan<T>> for DomainGreaterThan<T> {
    fn from(filter: GreaterThan<T>) -> Self {
        let GreaterThan(filter) = filter;
        Self(filter)
    }
}
