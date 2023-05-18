use fp_filter::LessEqual as DomainLessEqual;
use serde::{Deserialize, Serialize};

/// Serializable [less equal filter](DomainLessEqual) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct LessEqual<T>(pub T);

impl<T> From<DomainLessEqual<T>> for LessEqual<T> {
    fn from(filter: DomainLessEqual<T>) -> Self {
        let DomainLessEqual(filter) = filter;
        Self(filter)
    }
}

impl<T> From<LessEqual<T>> for DomainLessEqual<T> {
    fn from(filter: LessEqual<T>) -> Self {
        let LessEqual(filter) = filter;
        Self(filter)
    }
}
