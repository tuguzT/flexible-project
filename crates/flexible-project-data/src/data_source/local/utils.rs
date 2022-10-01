use fp_core::model::Node;
use mongodb::bson::Bson;

use crate::model::Id;

impl<Owner> From<Id<Owner>> for Bson
where
    Owner: ?Sized + Node,
{
    fn from(id: Id<Owner>) -> Self {
        Bson::String(id.to_string())
    }
}
