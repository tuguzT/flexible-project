//! Identifier definitions and utilities for the Flexible Project system model.

use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use derive_more::Display;
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn};

/// Type of identifier which are used to identify objects of the owner type.
#[repr(transparent)]
pub struct Id<Owner>
where
    Owner: ?Sized,
{
    id: String,
    _ph: PhantomData<fn() -> Owner>,
}

impl<Owner> Id<Owner>
where
    Owner: ?Sized,
{
    /// Creates new identifier from the string.
    pub fn new(id: String) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }

    /// Returns a string slice of this identifier.
    pub fn as_str(&self) -> &str {
        &self.id
    }

    /// Changes the owner of this identifier explicitly.
    pub fn change_owner<Other>(self) -> Id<Other>
    where
        Other: ?Sized,
    {
        self.id.into()
    }

    /// Erases the owner of this identifier explicitly,
    /// turning self into [`ErasedId`].
    pub fn erase(self) -> ErasedId {
        self.into()
    }
}

impl<Owner> PartialEq for Id<Owner>
where
    Owner: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<Owner> Eq for Id<Owner> where Owner: ?Sized {}

impl<Owner> PartialOrd for Id<Owner>
where
    Owner: ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Owner> Ord for Id<Owner>
where
    Owner: ?Sized,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<Owner> Clone for Id<Owner>
where
    Owner: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _ph: self._ph,
        }
    }
}

impl<Owner> Hash for Id<Owner>
where
    Owner: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Owner> Debug for Id<Owner>
where
    Owner: ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Id").field(&self.id).finish()
    }
}

impl<Owner> Display for Id<Owner>
where
    Owner: ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl<Owner> From<String> for Id<Owner>
where
    Owner: ?Sized,
{
    fn from(id: String) -> Self {
        Self::new(id)
    }
}

impl<Owner> From<Id<Owner>> for String
where
    Owner: ?Sized,
{
    fn from(id: Id<Owner>) -> Self {
        id.id
    }
}

/// Type of identifier with erased (unknown) owner.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ErasedId {
    id: String,
}

impl ErasedId {
    /// Creates new erased identifier from the string.
    pub fn new(id: String) -> Self {
        Self { id }
    }

    /// Returns a string slice of this identifier.
    pub fn as_str(&self) -> &str {
        &self.id
    }

    /// Sets the owner type for this identifier.
    pub fn with_owner<Owner>(self) -> Id<Owner>
    where
        Owner: ?Sized,
    {
        Id::new(self.id)
    }
}

impl From<String> for ErasedId {
    fn from(id: String) -> Self {
        Self::new(id)
    }
}

impl From<ErasedId> for String {
    fn from(id: ErasedId) -> Self {
        id.id
    }
}

impl<Owner> From<Id<Owner>> for ErasedId
where
    Owner: ?Sized,
{
    fn from(id: Id<Owner>) -> Self {
        Self::new(id.id)
    }
}

/// Filters for identifiers of the Flexible Project system.
#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct IdFilters<Owner: ?Sized> {
    /// Equality identifier filter.
    pub eq: Option<Equal<Id<Owner>>>,
    /// Inequality identifier filter.
    pub ne: Option<NotEqual<Id<Owner>>>,
    /// In identifier filter.
    pub r#in: Option<In<Id<Owner>>>,
    /// Not in identifier filter.
    pub nin: Option<NotIn<Id<Owner>>>,
}

impl<Owner> Default for IdFilters<Owner>
where
    Owner: ?Sized,
{
    fn default() -> Self {
        Self {
            eq: Default::default(),
            ne: Default::default(),
            r#in: Default::default(),
            nin: Default::default(),
        }
    }
}

impl<Owner> Clone for IdFilters<Owner>
where
    Owner: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            eq: self.eq.clone(),
            ne: self.ne.clone(),
            r#in: self.r#in.clone(),
            nin: self.nin.clone(),
        }
    }
}

impl<Owner> Hash for IdFilters<Owner>
where
    Owner: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eq.hash(state);
        self.ne.hash(state);
        self.r#in.hash(state);
        self.nin.hash(state);
    }
}

impl<Owner> PartialEq for IdFilters<Owner>
where
    Owner: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.eq == other.eq
            && self.ne == other.ne
            && self.r#in == other.r#in
            && self.nin == other.nin
    }
}

impl<Owner> Eq for IdFilters<Owner> where Owner: ?Sized {}

impl<Owner> PartialOrd for IdFilters<Owner>
where
    Owner: ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.eq.partial_cmp(&other.eq) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.ne.partial_cmp(&other.ne) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.r#in.partial_cmp(&other.r#in) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.nin.partial_cmp(&other.nin)
    }
}

impl<Owner> Ord for IdFilters<Owner>
where
    Owner: ?Sized,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.eq.cmp(&other.eq) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.ne.cmp(&other.ne) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.r#in.cmp(&other.r#in) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.nin.cmp(&other.nin)
    }
}
