use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use fp_core::model::{Id as CoreId, Identifiable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Serializable identifier of the owner object.
#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    id: Uuid,
    #[serde(skip)]
    _ph: PhantomData<Owner>,
}

impl<Owner> Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    /// Creates a random identifier.
    pub fn random() -> Self {
        Self {
            id: Uuid::new_v4(),
            _ph: PhantomData,
        }
    }

    /// Get inner representation of this identifier.
    pub fn as_inner(&self) -> &Uuid {
        &self.id
    }
}

impl<Owner> PartialEq for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Owner> Eq for Id<Owner> where Owner: ?Sized + Identifiable {}

impl<Owner> PartialOrd for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Owner> Ord for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<Owner> Clone for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _ph: self._ph,
        }
    }
}

impl<Owner> Hash for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Owner> CoreId<Owner> for Id<Owner> where Owner: ?Sized + Identifiable + 'static {}

impl<Owner> Debug for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Id").field(&self.id).finish()
    }
}

impl<Owner> Display for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl<Owner> From<Id<Owner>> for Uuid
where
    Owner: ?Sized + Identifiable,
{
    fn from(data: Id<Owner>) -> Self {
        data.id
    }
}

impl<Owner> From<Uuid> for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    /// Converts to [`Id`] of the owner type from the raw [`Uuid`].
    ///
    /// This conversion is safe, but result of using it when raw [`Uuid`]
    /// was obtained from another identifier with different owner type is *unspecified*.
    fn from(id: Uuid) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}
