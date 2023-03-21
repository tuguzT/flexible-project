//! Model of the user microservice domain layer.

use std::hash::{Hash, Hasher};

use fp_core::id::Id;

/// Type of user identifier.
pub type UserId = Id<User>;

/// Model of user in the system.
#[derive(Debug, Clone)]
pub struct User {
    /// Unique identifier of the user.
    pub id: UserId,
    /// Data of the user.
    pub data: UserData,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Data of the user in the system.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserData {
    /// Unique name of the user.
    pub name: String,
    /// Display name of the user.
    pub display_name: String,
    /// Role of the user.
    pub role: Role,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
}

/// Role of the user in the system.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Role {
    /// An ordinary user with no special rights.
    #[default]
    User,
    /// A moderator of the system which is responsible
    /// for public user content moderation.
    Moderator,
    /// An administrator of the system with special rights.
    Administrator,
}
