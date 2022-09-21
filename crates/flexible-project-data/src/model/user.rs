use fp_core::model::{Identifiable, User as CoreUser, UserRole as CoreUserRole};
use serde::{Deserialize, Serialize};

use crate::model::Id;

/// Serializable data of user.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct User {
    /// Identifier of the user.
    pub id: Id<Self>,
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    #[serde(with = "UserRole")]
    pub role: CoreUserRole,
}

impl User {
    /// Creates new user with provided id, name, email and role.
    pub fn new(id: Id<Self>, name: String, email: Option<String>, role: CoreUserRole) -> Self {
        Self {
            id,
            name,
            email,
            role,
        }
    }
}

impl Identifiable for User {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl CoreUser for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn role(&self) -> CoreUserRole {
        self.role
    }
}

/// Serializable variant of [`UserRole`] enum.
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[serde(remote = "CoreUserRole")]
pub enum UserRole {
    /// An ordinary user of the system.
    #[default]
    User,
    /// Role of a moderator of the system.
    Moderator,
    /// Role of an administrator of the system.
    Administrator,
}
