//! Model of the user microservice domain layer.

/// User model in the system.
pub struct User {
    /// Unique name of the user.
    pub name: String,
    /// Display name of the user.
    pub display_name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user.
    pub role: Role,
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

/// Credentials of the user which are used to authenticate a specific user.
pub struct Credentials {
    /// Unique name of the user.
    pub name: String,
    /// Password of the user.
    pub password: String,
}
