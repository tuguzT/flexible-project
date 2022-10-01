use fp_core::model::{Node, User as CoreUser};
use mongodb::bson::Bson;
use mongodb::{Collection, Database};

use crate::model::{Id, User};

impl<Owner> From<Id<Owner>> for Bson
where
    Owner: ?Sized + Node,
{
    fn from(id: Id<Owner>) -> Self {
        Bson::String(id.to_string())
    }
}

pub trait UserCollection {
    type User: CoreUser;

    fn user_collection(self) -> Collection<Self::User>;
}

impl UserCollection for Database {
    type User = User;

    fn user_collection(self) -> Collection<Self::User> {
        self.collection("users")
    }
}
