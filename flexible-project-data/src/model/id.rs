use std::marker::PhantomData;
use std::{fmt::Display, hash::Hash};

use fp_core::model::{Id as DomainId, Identifiable};
use serde::{Deserialize, Serialize};

/// Serializable identifier of the owner object.
#[derive(Serialize, Deserialize)]
pub struct Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    id: String,
    _ph: PhantomData<Owner>,
}

impl<Owner> PartialEq for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self._ph == other._ph
    }
}

impl<Owner> Eq for Id<Owner> where Owner: ?Sized + Identifiable {}

impl<Owner> Clone for Id<Owner>
where
    Owner: ?Sized + Identifiable,
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
    Owner: ?Sized + Identifiable,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self._ph.hash(state);
    }
}

impl<Owner> DomainId<Owner> for Id<Owner> where Owner: ?Sized + Identifiable + 'static {}

impl<Owner> Display for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

impl<Owner> From<String> for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: String) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}

impl<Owner> From<&str> for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: &str) -> Self {
        Self {
            id: id.to_string(),
            _ph: PhantomData,
        }
    }
}
