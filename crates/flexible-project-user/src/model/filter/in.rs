use fp_filter::In as DomainIn;
use serde::{Deserialize, Serialize};

/// Serializable [in filter](DomainIn) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct In<T>(pub T)
where
    T: IntoIterator;

impl<T> From<DomainIn<T>> for In<T>
where
    T: IntoIterator,
{
    fn from(filter: DomainIn<T>) -> Self {
        let DomainIn(filter) = filter;
        Self(filter)
    }
}

impl<T> From<In<T>> for DomainIn<T>
where
    T: IntoIterator,
{
    fn from(filter: In<T>) -> Self {
        let In(filter) = filter;
        Self(filter)
    }
}
