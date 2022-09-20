use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use fp_core::model::{Id, Identifiable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Serializable identifier of the owner object.
#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    id: Uuid,
    #[serde(skip)]
    _ph: PhantomData<Owner>,
}

impl<Owner> IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    /// Creates a random identifier.
    pub fn new() -> Self {
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

impl<Owner> PartialEq for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Owner> Eq for IdData<Owner> where Owner: ?Sized + Identifiable {}

impl<Owner> PartialOrd for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Owner> Ord for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<Owner> Clone for IdData<Owner>
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

impl<Owner> Hash for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Owner> Id<Owner> for IdData<Owner> where Owner: ?Sized + Identifiable + 'static {}

impl<Owner> Debug for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IdData").field(&self.id).finish()
    }
}

impl<Owner> Display for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id)
    }
}

impl<Owner> From<IdData<Owner>> for Uuid
where
    Owner: ?Sized + Identifiable,
{
    fn from(data: IdData<Owner>) -> Self {
        data.id
    }
}

impl<Owner> From<Uuid> for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: Uuid) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}
