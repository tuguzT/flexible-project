use fp_filter::Between as DomainBetween;
use serde::{Deserialize, Serialize};

/// Serializable [between filter](DomainBetween) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Between<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<DomainBetween<T>> for Between<T> {
    fn from(filter: DomainBetween<T>) -> Self {
        let DomainBetween { min, max } = filter;
        Self { min, max }
    }
}

impl<T> From<Between<T>> for DomainBetween<T> {
    fn from(filter: Between<T>) -> Self {
        let Between { min, max } = filter;
        Self { min, max }
    }
}
