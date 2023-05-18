use fp_filter::LessThan as DomainLessThan;
use serde::{Deserialize, Serialize};

/// Serializable [less than filter](DomainLessThan) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct LessThan<T>(pub T);

impl<T> From<DomainLessThan<T>> for LessThan<T> {
    fn from(filter: DomainLessThan<T>) -> Self {
        let DomainLessThan(filter) = filter;
        Self(filter)
    }
}

impl<T> From<LessThan<T>> for DomainLessThan<T> {
    fn from(filter: LessThan<T>) -> Self {
        let LessThan(filter) = filter;
        Self(filter)
    }
}
