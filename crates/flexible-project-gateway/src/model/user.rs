//! User data model of the gateway service.

use async_graphql::{Enum, InputObject, Object, SimpleObject, ID};

/// Query object of users of the Flexible Project system.
#[derive(Debug, Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Filters all users of the system.
    pub async fn users(&self, filters: UserFilters) -> Vec<User> {
        let _ = filters;
        None.unwrap()
    }
}

/// Mutation object of users of the Flexible Project system.
#[derive(Debug, Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Creates new user with provided name in the system.
    pub async fn create_user(&self, name: String) -> User {
        let _ = name;
        None.unwrap()
    }

    /// Updates properties of the user by provided identifier with provided data.
    pub async fn update_user(&self, id: ID, update: UpdateUser) -> User {
        let _ = (id, update);
        None.unwrap()
    }

    /// Deletes user from the system by provided identifier.
    pub async fn delete_user(&self, id: ID) -> User {
        let _ = id;
        None.unwrap()
    }
}

/// User properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    /// Unique identifier of the user.
    pub id: ID,
    /// Unique name of the user.
    pub name: String,
    /// Display name of the user.
    pub display_name: String,
    /// Role of the user.
    pub role: UserRole,
    /// Optional email of the user.
    pub email: Option<String>,
    /// Optional avatar of the user.
    pub avatar_url: Option<String>,
}

/// Filters of users of the Flexible Project system.
#[derive(Debug, InputObject)]
pub struct UserFilters {
    /// Identifier filter of the user.
    pub id: Option<ID>,
}

/// Data of the user to update.
#[derive(Debug, InputObject)]
pub struct UpdateUser {
    /// Name of the user to update, if present.
    pub name: Option<String>,
    /// Display name of the user to update, if present.
    pub display_name: Option<String>,
    /// Email of the user to update, if present.
    pub email: Option<Option<String>>,
    /// Avatar of the user to update, if present.
    pub avatar_url: Option<Option<String>>,
}

/// Role of the user in the Flexible Project system.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UserRole {
    /// An ordinary user with no special rights.
    User,
    /// A moderator of the system which is responsible
    /// for public user content moderation.
    Moderator,
    /// An administrator of the system with special rights.
    Administrator,
}
