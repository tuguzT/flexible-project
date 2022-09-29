use crate::model::Node;

/// Trait for the users of the Flexible Project system.
pub trait User: Node {
    /// Get a unique user name.
    fn name(&self) -> &str;

    /// Get a unique email of the user.
    fn email(&self) -> Option<&str>;

    /// Get a role of the user in the system.
    fn role(&self) -> UserRole;
}

/// Represents role of the user in the Flexible Project system.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum UserRole {
    /// An ordinary user of the system.
    #[default]
    User,
    /// Role of a moderator of the system.
    Moderator,
    /// Role of an administrator of the system.
    Administrator,
}

/// Credentials of the user such as username and password
/// used to authenticate a user.
pub trait UserCredentials {
    /// Get a user name provided by the user.
    fn name(&self) -> &str;

    /// Get a password provided by the user.
    fn password(&self) -> &str;
}

/// Filters to be applied on user filtering.
#[derive(Default)]
pub struct UserFilters;
