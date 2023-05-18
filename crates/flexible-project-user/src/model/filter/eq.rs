use fp_filter::Equal as DomainEqual;
use serde::{Deserialize, Serialize};

/// Serializable [equality filter](DomainEqual) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Equal<T>(pub T);

impl<T> From<DomainEqual<T>> for Equal<T> {
    fn from(filter: DomainEqual<T>) -> Self {
        let DomainEqual(filter) = filter;
        Self(filter)
    }
}

impl<T> From<Equal<T>> for DomainEqual<T> {
    fn from(filter: Equal<T>) -> Self {
        let Equal(filter) = filter;
        Self(filter)
    }
}
