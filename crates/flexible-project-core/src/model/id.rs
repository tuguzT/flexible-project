use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

use derive_more::Display;

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

    /// Changes the owner of this identifier explicitly.
    pub fn change_owner<Other>(self) -> Id<Other>
    where
        Other: ?Sized,
    {
        Id::new(self.id)
    }

    /// Erases the owner of this identifier explicitly,
    /// turning self into [`ErasedId`].
    pub fn erase(self) -> ErasedId {
        self.into()
    }
}

impl<Owner> Deref for Id<Owner>
where
    Owner: ?Sized,
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.id.deref()
    }
}

impl<Owner> PartialEq for Id<Owner>
where
    Owner: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

/// Type of identifier with erased (unknown) owner.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErasedId {
    id: String,
}

impl ErasedId {
    /// Creates new erased identifier from the string.
    pub fn new(id: String) -> Self {
        Self { id }
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

impl<Owner> From<Id<Owner>> for ErasedId
where
    Owner: ?Sized,
{
    fn from(id: Id<Owner>) -> Self {
        Self::new(id.id)
    }
}
