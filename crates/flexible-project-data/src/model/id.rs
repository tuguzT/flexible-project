use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

use fp_core::model::{Id as CoreId, Node};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Serializable identifier of the owner object.
#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Id<Owner>
where
    Owner: ?Sized + Node,
{
    id: Uuid,
    #[serde(skip)]
    _ph: PhantomData<Owner>,
}

impl<Owner> Id<Owner>
where
    Owner: ?Sized + Node,
{
    /// Creates a random identifier.
    pub fn random() -> Self {
        Self {
            id: Uuid::new_v4(),
            _ph: PhantomData,
        }
    }
}

impl<Owner> PartialEq for Id<Owner>
where
    Owner: ?Sized + Node,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Owner> Eq for Id<Owner> where Owner: ?Sized + Node {}

impl<Owner> PartialOrd for Id<Owner>
where
    Owner: ?Sized + Node,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Owner> Ord for Id<Owner>
where
    Owner: ?Sized + Node,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<Owner> Clone for Id<Owner>
where
    Owner: ?Sized + Node,
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
    Owner: ?Sized + Node,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Owner> CoreId<Owner> for Id<Owner> where Owner: ?Sized + Node + 'static {}

impl<Owner> Debug for Id<Owner>
where
    Owner: ?Sized + Node,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Id").field(&self.id).finish()
    }
}

impl<Owner> Display for Id<Owner>
where
    Owner: ?Sized + Node,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl<Owner> FromStr for Id<Owner>
where
    Owner: ?Sized + Node,
{
    type Err = uuid::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let this = Self {
            id: Uuid::parse_str(input)?,
            _ph: PhantomData,
        };
        Ok(this)
    }
}
