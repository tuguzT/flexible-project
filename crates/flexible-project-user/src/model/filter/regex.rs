use fp_filter::Regex as DomainRegex;
use serde::{Deserialize, Serialize};

/// Serializable [regex filter](DomainRegex) of the backend.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Regex<R>(pub R);

impl<R> From<DomainRegex<R>> for Regex<R> {
    fn from(filter: DomainRegex<R>) -> Self {
        let DomainRegex(filter) = filter;
        Self(filter)
    }
}

impl<R> From<Regex<R>> for DomainRegex<R> {
    fn from(filter: Regex<R>) -> Self {
        let Regex(filter) = filter;
        Self(filter)
    }
}
