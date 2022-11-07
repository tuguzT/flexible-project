use fp_core::model::id::Id;
use mongodb::bson::uuid::{Error, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IdData {
    uuid: Uuid,
}

impl<Owner> From<IdData> for Id<Owner>
where
    Owner: ?Sized,
{
    fn from(id: IdData) -> Self {
        id.uuid.to_string().into()
    }
}

impl<Owner> TryFrom<Id<Owner>> for IdData
where
    Owner: ?Sized,
{
    type Error = Error;

    fn try_from(id: Id<Owner>) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(String::from(id))?;
        Ok(Self { uuid })
    }
}
