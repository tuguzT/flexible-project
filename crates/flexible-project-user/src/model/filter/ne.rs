use fp_filter::NotEqual as DomainNotEqual;
use serde::{Deserialize, Serialize};

/// Serializable [inequality filter](DomainNotEqual) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct NotEqual<T>(pub T);

impl<T> From<DomainNotEqual<T>> for NotEqual<T> {
    fn from(filter: DomainNotEqual<T>) -> Self {
        let DomainNotEqual(filter) = filter;
        Self(filter)
    }
}

impl<T> From<NotEqual<T>> for DomainNotEqual<T> {
    fn from(filter: NotEqual<T>) -> Self {
        let NotEqual(filter) = filter;
        Self(filter)
    }
}
