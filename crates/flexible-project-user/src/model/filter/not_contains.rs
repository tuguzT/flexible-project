use fp_filter::NotContains as DomainNotContains;
use serde::{Deserialize, Serialize};

/// Serializable [not contains filter](DomainNotContains) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct NotContains<T>(pub T);

impl<T> From<DomainNotContains<T>> for NotContains<T> {
    fn from(filter: DomainNotContains<T>) -> Self {
        let DomainNotContains(filter) = filter;
        Self(filter)
    }
}

impl<T> From<NotContains<T>> for DomainNotContains<T> {
    fn from(filter: NotContains<T>) -> Self {
        let NotContains(filter) = filter;
        Self(filter)
    }
}
