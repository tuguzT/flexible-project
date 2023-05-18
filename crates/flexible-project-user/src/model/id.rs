use std::{borrow::Cow, marker::PhantomData};

use derive_more::Display;
use fp_core::id::{ErasedId as CoreErasedId, ErasedIdFilters as DomainErasedIdFilters};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn};

/// Serializable [erased identifier](CoreErasedId) of the system.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ErasedId(String);

impl From<CoreErasedId> for ErasedId {
    fn from(id: CoreErasedId) -> Self {
        let id = id.into_inner();
        Self(id)
    }
}

impl From<ErasedId> for CoreErasedId {
    fn from(id: ErasedId) -> Self {
        let ErasedId(id) = id;
        CoreErasedId::new(id)
    }
}

/// Filters for identifier of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct ErasedIdFilters {
    /// Equality identifier filter.
    pub eq: Option<Equal<ErasedId>>,
    /// Inequality identifier filter.
    pub ne: Option<NotEqual<ErasedId>>,
    /// In identifier filter.
    pub r#in: Option<In<Vec<ErasedId>>>,
    /// Not in identifier filter.
    pub nin: Option<NotIn<Vec<ErasedId>>>,
}

impl From<DomainErasedIdFilters<'_>> for ErasedIdFilters {
    fn from(filters: DomainErasedIdFilters<'_>) -> Self {
        let DomainErasedIdFilters {
            owner: _,
            eq,
            ne,
            r#in,
            nin,
        } = filters;
        Self {
            eq: eq.map(|id| Equal(id.0.into_owned().into())),
            ne: ne.map(|id| NotEqual(id.0.into_owned().into())),
            r#in: r#in.map(|ids| {
                let cow_slice = ids.0;
                In(cow_slice.0.iter().cloned().map(Into::into).collect())
            }),
            nin: nin.map(|ids| {
                let cow_slice = ids.0;
                NotIn(cow_slice.0.iter().cloned().map(Into::into).collect())
            }),
        }
    }
}

impl From<ErasedIdFilters> for DomainErasedIdFilters<'_> {
    fn from(filters: ErasedIdFilters) -> Self {
        let ErasedIdFilters { eq, ne, r#in, nin } = filters;
        Self {
            owner: PhantomData,
            eq: eq.map(|Equal(id)| Equal(Cow::Owned(id.into())).into()),
            ne: ne.map(|NotEqual(id)| NotEqual(Cow::Owned(id.into())).into()),
            r#in: r#in.map(|In(ids)| {
                let ids: Vec<_> = ids.into_iter().map(Into::into).collect();
                In(ids.into()).into()
            }),
            nin: nin.map(|NotIn(ids)| {
                let ids: Vec<_> = ids.into_iter().map(Into::into).collect();
                NotIn(ids.into()).into()
            }),
        }
    }
}
