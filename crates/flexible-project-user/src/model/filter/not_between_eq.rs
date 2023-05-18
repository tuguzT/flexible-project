use fp_filter::NotBetweenEqual as DomainNotBetweenEqual;
use serde::{Deserialize, Serialize};

/// Serializable [not between equal filter](DomainNotBetweenEqual) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct NotBetweenEqual<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<DomainNotBetweenEqual<T>> for NotBetweenEqual<T> {
    fn from(filter: DomainNotBetweenEqual<T>) -> Self {
        let DomainNotBetweenEqual { min, max } = filter;
        Self { min, max }
    }
}

impl<T> From<NotBetweenEqual<T>> for DomainNotBetweenEqual<T> {
    fn from(filter: NotBetweenEqual<T>) -> Self {
        let NotBetweenEqual { min, max } = filter;
        Self { min, max }
    }
}
