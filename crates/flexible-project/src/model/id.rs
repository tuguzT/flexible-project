use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use fp_core::model::Identifiable;
use fp_data::model::IdData;

/// GraphQL scalar identifier of the object.
pub struct Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    id: String,
    _ph: PhantomData<Owner>,
}

impl<Owner> Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    /// Get a string representation of this identifier.
    pub fn as_str(&self) -> &str {
        &self.id
    }
}

#[Scalar]
impl<Owner> ScalarType for Id<Owner>
where
    Owner: ?Sized + Identifiable + Send + Sync,
{
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(id) => Ok(Self {
                id,
                _ph: PhantomData,
            }),
            actual => Err(InputValueError::expected_type(actual)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.id.clone())
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
    }
}

impl<Owner> fp_core::model::Id<Owner> for Id<Owner> where Owner: ?Sized + Identifiable + 'static {}

impl<Owner> Debug for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("IdData").field(&self.id).finish()
    }
}

impl<Owner> Display for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

impl<Owner> From<Id<Owner>> for String
where
    Owner: ?Sized + Identifiable,
{
    fn from(data: Id<Owner>) -> Self {
        data.id
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

impl<Owner> From<IdData<Owner>> for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: IdData<Owner>) -> Self {
        Self {
            id: id.into(),
            _ph: PhantomData,
        }
    }
}

impl<Owner> From<Id<Owner>> for IdData<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: Id<Owner>) -> Self {
        let id: String = id.into();
        Self::from(id)
    }
}
