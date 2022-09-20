use fp_core::model::{Identifiable, User, UserRole};
use serde::{Deserialize, Serialize};

use crate::model::IdData;

/// Serializable data of user.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserData {
    /// Identifier of the user.
    pub id: IdData<Self>,
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    #[serde(with = "UserRoleData")]
    pub role: UserRole,
}

impl UserData {
    /// Creates new user with provided id, name, email and role.
    pub fn new(id: IdData<Self>, name: String, email: Option<String>, role: UserRole) -> Self {
        Self {
            id,
            name,
            email,
            role,
        }
    }
}

impl Identifiable for UserData {
    type Id = IdData<Self>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl User for UserData {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn role(&self) -> UserRole {
        self.role
    }
}

/// Serializable variant of [`UserRole`] enum.
#[derive(
    Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[serde(remote = "UserRole")]
pub enum UserRoleData {
    /// An ordinary user of the system.
    #[default]
    User,
    /// Role of a moderator of the system.
    Moderator,
    /// Role of an administrator of the system.
    Administrator,
}
