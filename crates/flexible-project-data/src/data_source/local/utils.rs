use mongodb::{Collection, Database};

use super::model::UserData;

pub trait UserCollection {
    fn user_collection(self) -> Collection<UserData>;
}

impl UserCollection for Database {
    fn user_collection(self) -> Collection<UserData> {
        self.collection("users")
    }
}
