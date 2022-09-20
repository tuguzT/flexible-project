use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use fp_core::model::{Id as CoreId, Identifiable};
use fp_data::model::Id as DataId;
use uuid::Uuid;

/// GraphQL scalar identifier of the object.
#[derive(Default)]
pub struct Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    id: Uuid,
    _ph: PhantomData<Owner>,
}

impl<Owner> Id<Owner>
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

#[Scalar]
impl<Owner> ScalarType for Id<Owner>
where
    Owner: ?Sized + Identifiable + Send + Sync,
{
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(id) => {
                let id = match id.parse::<Uuid>() {
                    Ok(id) => id,
                    Err(err) => return Err(InputValueError::custom(err)),
                };
                Ok(Self::from(id))
            }
            actual => Err(InputValueError::expected_type(actual)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.id.to_string())
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
    fn from(id: Uuid) -> Self {
        Self {
            id,
            _ph: PhantomData,
        }
    }
}

impl<Owner> From<DataId<Owner>> for Id<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: DataId<Owner>) -> Self {
        let id: Uuid = id.into();
        Self::from(id)
    }
}

impl<Owner> From<Id<Owner>> for DataId<Owner>
where
    Owner: ?Sized + Identifiable,
{
    fn from(id: Id<Owner>) -> Self {
        let id: Uuid = id.into();
        Self::from(id)
    }
}
