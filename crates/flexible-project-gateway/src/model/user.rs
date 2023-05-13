//! User data model of the gateway service.

use async_graphql::{Enum, InputObject, Object, SimpleObject, ID};

/// Query object of users of the Flexible Project system.
#[derive(Debug, Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Filters all users of the system.
    pub async fn filter(&self, filters: UserFilters) -> Vec<User> {
        let _ = filters;
        None.unwrap()
    }

    /// Creates new user with provided name in the system.
    pub async fn create(&self, name: String) -> User {
        let _ = name;
        None.unwrap()
    }

    /// Updates name of the user by provided identifier.
    pub async fn update_name(&self, id: ID, name: String) -> User {
        let _ = (id, name);
        None.unwrap()
    }

    /// Updates display name of the user by provided identifier.
    pub async fn update_display_name(&self, id: ID, display_name: String) -> User {
        let _ = (id, display_name);
        None.unwrap()
    }

    /// Updates email of the user by provided identifier.
    pub async fn update_email(&self, id: ID, email: Option<String>) -> User {
        let _ = (id, email);
        None.unwrap()
    }

    /// Updates avatar of the user by provided identifier.
    pub async fn update_avatar(&self, id: ID, avatar_url: Option<String>) -> User {
        let _ = (id, avatar_url);
        None.unwrap()
    }

    /// Deletes user from the system by provided identifier.
    pub async fn delete(&self, id: ID) -> User {
        let _ = id;
        None.unwrap()
    }
}

/// User of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    /// Identifier of the user.
    pub id: ID,
    /// Unique name of the user.
    pub name: String,
    /// Display name of the user.
    pub display_name: String,
    /// Role of the user.
    pub role: Role,
    /// Optional email of the user.
    pub email: Option<String>,
    /// Optional avatar URL of the user.
    pub avatar_url: Option<String>,
}

/// Filters of users of the Flexible Project system.
#[derive(Debug, InputObject)]
pub struct UserFilters {
    /// Identifier filter of the user.
    pub id: Option<ID>,
}

/// Role of the user in the Flexible Project system.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    /// An ordinary user with no special rights.
    User,
    /// A moderator of the system which is responsible
    /// for public user content moderation.
    Moderator,
    /// An administrator of the system with special rights.
    Administrator,
}
