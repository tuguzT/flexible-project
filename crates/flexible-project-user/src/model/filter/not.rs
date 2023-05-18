use fp_filter::Not as DomainNot;
use serde::{Deserialize, Serialize};

/// Serializable [negation filter](DomainNot) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Not<F>(pub F);

impl<T> From<DomainNot<T>> for Not<T> {
    fn from(filter: DomainNot<T>) -> Self {
        let DomainNot(filter) = filter;
        Self(filter)
    }
}

impl<T> From<Not<T>> for DomainNot<T> {
    fn from(filter: Not<T>) -> Self {
        let Not(filter) = filter;
        Self(filter)
    }
}
