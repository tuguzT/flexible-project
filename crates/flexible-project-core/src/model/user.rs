#![allow(missing_docs)]

use derive_more::{Display, From, IsVariant, Unwrap};

use crate::model::Id;

/// Data of user of the Flexible Project system.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

/// Represents role of the user in the Flexible Project system.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, IsVariant, Unwrap)]
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
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserCredentials {
    /// Name of the user.
    pub name: String,
    /// Password of the user.
    pub password: String,
}

/// Filters to be applied on user filtering.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UserFilters {
    /// Identifiers to be filtered in search query.
    pub ids: Vec<Id<User>>,
}

impl UserFilters {
    /// Checks if user filters are completely empty.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

/// User access token which is required to access non-public system resources.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct UserToken {
    /// User token general representation.
    pub token: String,
}

#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserTokenClaims {
    /// Identifier of the user.
    pub id: Id<User>,
}
