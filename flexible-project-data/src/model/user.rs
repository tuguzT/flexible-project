use fp_core::model::{Identifiable, User as DomainUser, UserRole};
use serde::{Deserialize, Serialize};

use crate::model::Id;

/// Serializable data of user.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct User {
    id: Id<Self>,
    name: String,
    email: Option<String>,
    role: SerializableUserRole,
}

impl User {
    /// Creates new user with provided id, name, email and role.
    pub fn new(id: Id<Self>, name: String, email: Option<String>, role: UserRole) -> Self {
        Self {
            id,
            name,
            email,
            role: role.into(),
        }
    }
}

impl Identifiable for User {
    type Id = Id<Self>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl DomainUser for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn role(&self) -> UserRole {
        self.role.into()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum SerializableUserRole {
    User,
    Moderator,
    Administrator,
}

impl From<SerializableUserRole> for UserRole {
    fn from(role: SerializableUserRole) -> Self {
        match role {
            SerializableUserRole::User => UserRole::User,
            SerializableUserRole::Moderator => UserRole::Moderator,
            SerializableUserRole::Administrator => UserRole::Administrator,
        }
    }
}

impl From<UserRole> for SerializableUserRole {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::User => SerializableUserRole::User,
            UserRole::Moderator => SerializableUserRole::Moderator,
            UserRole::Administrator => SerializableUserRole::Administrator,
        }
    }
}
