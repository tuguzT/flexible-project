use fp_filter::NotIn as DomainNotIn;
use serde::{Deserialize, Serialize};

/// Serializable [not in filter](DomainNotIn) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct NotIn<T>(pub T);

impl<T> From<DomainNotIn<T>> for NotIn<T> {
    fn from(filter: DomainNotIn<T>) -> Self {
        let DomainNotIn(filter) = filter;
        Self(filter)
    }
}

impl<T> From<NotIn<T>> for DomainNotIn<T> {
    fn from(filter: NotIn<T>) -> Self {
        let NotIn(filter) = filter;
        Self(filter)
    }
}
