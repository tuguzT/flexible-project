use fp_filter::BetweenEqual as DomainBetweenEqual;
use serde::{Deserialize, Serialize};

/// Serializable [between equal filter](DomainBetweenEqual) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct BetweenEqual<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<DomainBetweenEqual<T>> for BetweenEqual<T> {
    fn from(filter: DomainBetweenEqual<T>) -> Self {
        let DomainBetweenEqual { min, max } = filter;
        Self { min, max }
    }
}

impl<T> From<BetweenEqual<T>> for DomainBetweenEqual<T> {
    fn from(filter: BetweenEqual<T>) -> Self {
        let BetweenEqual { min, max } = filter;
        Self { min, max }
    }
}
