use async_graphql::{Enum, InputObject, SimpleObject};
use fp_core::model::Identifiable;
use uuid::Uuid;

use crate::model::Id;

/// GraphQL enumeration which represents role of user in the Flexible Project system.
#[derive(Enum, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[graphql(remote = "fp_core::model::UserRole")]
pub enum UserRole {
    /// An ordinary user of the system.
    #[default]
    User,
    /// Role of a moderator of the system.
    Moderator,
    /// Role of an administrator of the system.
    Administrator,
}

/// GraphQL output object which represents data of user in the Flexible Project system.
#[derive(SimpleObject)]
pub struct User {
    /// Identifier of the user.
    pub id: Id<Self>,
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    pub role: UserRole,
}

impl Identifiable for User {
    type Id = Id<Self>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl fp_core::model::User for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn role(&self) -> fp_core::model::UserRole {
        self.role.into()
    }
}

impl From<fp_data::model::UserData> for User {
    fn from(user_data: fp_data::model::UserData) -> Self {
        Self {
            id: Uuid::from(user_data.id).into(),
            name: user_data.name,
            email: user_data.email,
            role: user_data.role.into(),
        }
    }
}

impl From<User> for fp_data::model::UserData {
    fn from(user_data: User) -> Self {
        Self {
            id: Uuid::from(user_data.id).into(),
            name: user_data.name,
            email: user_data.email,
            role: user_data.role.into(),
        }
    }
}

/// GraphQL input object with necessary data for creating new user.
#[derive(InputObject)]
pub struct NewUser {
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
}
