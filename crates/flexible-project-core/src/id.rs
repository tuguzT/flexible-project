//! Data model of identifier of the backend.

use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
};

use derive_more::Display;
use typed_builder::TypedBuilder;

use crate::filter::{Equal, Filter, In, NotEqual, NotIn};

/// Type of identifier which are used to identify objects of the owner type.
pub struct Id<Owner> {
    inner: String,
    owner: PhantomData<fn() -> Owner>,
}

impl<Owner> Id<Owner> {
    /// Creates new identifier from the string.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            inner: id.into(),
            owner: PhantomData,
        }
    }

    /// Extracts a string slice from the entire identifier.
    pub fn as_str(&self) -> &str {
        let Self { inner, .. } = self;
        inner.as_str()
    }

    /// Converts an identifier into a [`String`].
    pub fn into_inner(self) -> String {
        let Self { inner, .. } = self;
        inner
    }

    /// Changes the owner of an identifier explicitly.
    pub fn change_owner<Other>(self) -> Id<Other> {
        let Self { inner, .. } = self;
        Id::new(inner)
    }

    /// Erases the owner of an identifier explicitly, turning it into [`ErasedId`].
    pub fn erase(self) -> ErasedId {
        let Self { inner, .. } = self;
        ErasedId::new(inner)
    }
}

impl<Owner> PartialEq for Id<Owner> {
    fn eq(&self, other: &Self) -> bool {
        let Self { inner, .. } = self;
        let Self { inner: other, .. } = other;
        inner.eq(other)
    }
}

impl<Owner> Eq for Id<Owner> {}

impl<Owner> PartialOrd for Id<Owner> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let Self { inner, .. } = self;
        let Self { inner: other, .. } = other;
        inner.partial_cmp(other)
    }
}

impl<Owner> Ord for Id<Owner> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Self { inner, .. } = self;
        let Self { inner: other, .. } = other;
        inner.cmp(other)
    }
}

impl<Owner> Clone for Id<Owner> {
    fn clone(&self) -> Self {
        let Self { ref inner, owner } = *self;
        let inner = inner.clone();
        Self { inner, owner }
    }
}

impl<Owner> Hash for Id<Owner> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let Self { inner, .. } = self;
        inner.hash(state);
    }
}

impl<Owner> Debug for Id<Owner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { inner, .. } = self;
        f.debug_tuple("Id").field(inner).finish()
    }
}

impl<Owner> Display for Id<Owner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { inner, .. } = self;
        Display::fmt(inner, f)
    }
}

/// Type of identifier with erased (unknown) owner.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErasedId(String);

impl ErasedId {
    /// Creates new erased identifier from the string.
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self(id)
    }

    /// Extracts a string slice from the entire identifier.
    pub fn as_str(&self) -> &str {
        let Self(id) = self;
        id.as_str()
    }

    /// Converts an identifier into a [`String`].
    pub fn into_inner(self) -> String {
        let Self(id) = self;
        id
    }

    /// Sets the owner type for an identifier, turning it into [`Id`].
    pub fn with_owner<Owner>(self) -> Id<Owner> {
        let Self(id) = self;
        Id::new(id)
    }
}

/// Filters for identifiers of the backend.
#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct IdFilters<'a, Owner: 'a> {
    /// Equality identifier filter.
    pub eq: Option<Equal<'a, Id<Owner>>>,
    /// Inequality identifier filter.
    pub ne: Option<NotEqual<'a, Id<Owner>>>,
    /// In identifier filter.
    pub r#in: Option<In<'a, Id<Owner>>>,
    /// Not in identifier filter.
    pub nin: Option<NotIn<'a, Id<Owner>>>,
}

impl<Owner> Filter for IdFilters<'_, Owner> {
    type Input = Id<Owner>;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self { eq, ne, r#in, nin } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}

impl<Owner> Default for IdFilters<'_, Owner> {
    fn default() -> Self {
        Self {
            eq: Default::default(),
            ne: Default::default(),
            r#in: Default::default(),
            nin: Default::default(),
        }
    }
}

impl<Owner> Clone for IdFilters<'_, Owner> {
    fn clone(&self) -> Self {
        let Self { eq, ne, r#in, nin } = self;
        Self {
            eq: eq.clone(),
            ne: ne.clone(),
            r#in: r#in.clone(),
            nin: nin.clone(),
        }
    }
}
