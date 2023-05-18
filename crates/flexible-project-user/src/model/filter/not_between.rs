use fp_filter::NotBetween as DomainNotBetween;
use serde::{Deserialize, Serialize};

/// Serializable [not between filter](DomainNotBetween) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct NotBetween<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<DomainNotBetween<T>> for NotBetween<T> {
    fn from(filter: DomainNotBetween<T>) -> Self {
        let DomainNotBetween { min, max } = filter;
        Self { min, max }
    }
}

impl<T> From<NotBetween<T>> for DomainNotBetween<T> {
    fn from(filter: NotBetween<T>) -> Self {
        let NotBetween { min, max } = filter;
        Self { min, max }
    }
}
