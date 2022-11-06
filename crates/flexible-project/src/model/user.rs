//! User utilities for the Flexible Project server model.

#![allow(missing_docs)]

use async_graphql::{ComplexObject, Enum, InputObject, SimpleObject, ID};
use derive_more::{Display, From, IsVariant, Unwrap};
use fp_core::model::id::Id;
use fp_core::model::user::{
    User as CoreUser, UserCredentials as CoreUserCredentials, UserFilters as CoreUserFilters,
    UserRole as CoreUserRole, UserToken as CoreUserToken, UsernameFilters as CoreUsernameFilters,
};

use crate::model::id::IdFilters;

/// Role of user in the Flexible Project system.
#[derive(
    Enum, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, IsVariant, Unwrap,
)]
#[graphql(remote = "CoreUserRole")]
pub enum UserRole {
    /// An ordinary user of the system.
    #[default]
    User,
    /// Role of a moderator of the system.
    Moderator,
    /// Role of an administrator of the system.
    Administrator,
}

/// User data in the Flexible Project system.
#[derive(SimpleObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[graphql(complex)]
pub struct User {
    /// Unique identifier of the user.
    #[graphql(skip)]
    pub id: Id<Self>,
    /// Unique name of the user.
    pub name: String,
    /// Display name of the user which is not unique.
    pub display_name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    pub role: UserRole,
}

#[ComplexObject]
impl User {
    /// Unique identifier of the user.
    pub async fn id(&self) -> ID {
        self.id.clone().into()
    }
}

impl From<CoreUser> for User {
    fn from(user: CoreUser) -> Self {
        Self {
            id: user.id.change_owner(),
            name: user.name,
            display_name: user.display_name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

impl From<User> for CoreUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.change_owner(),
            name: user.name,
            display_name: user.display_name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

/// User credentials in the Flexible Project system.
#[derive(InputObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserCredentials {
    /// Name of the user.
    pub name: String,
    /// Password of the user.
    #[graphql(secret)]
    pub password: String,
}

impl From<CoreUserCredentials> for UserCredentials {
    fn from(credentials: CoreUserCredentials) -> Self {
        Self {
            name: credentials.name,
            password: credentials.password,
        }
    }
}

impl From<UserCredentials> for CoreUserCredentials {
    fn from(credentials: UserCredentials) -> Self {
        Self {
            name: credentials.name,
            password: credentials.password,
        }
    }
}

/// User input data in the Flexible Project system.
#[derive(InputObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateUser {
    /// Unique identifier of the user.
    pub id: ID,
    /// Unique name of the user.
    pub name: String,
    /// Display name of the user which is not unique.
    pub display_name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    pub role: UserRole,
}

impl From<CoreUser> for UpdateUser {
    fn from(user: CoreUser) -> Self {
        Self {
            id: user.id.into(),
            name: user.name,
            display_name: user.display_name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

impl From<UpdateUser> for CoreUser {
    fn from(user: UpdateUser) -> Self {
        Self {
            id: user.id.to_string().into(),
            name: user.name,
            display_name: user.display_name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

/// User access token which is required to access non-public system resources.
#[derive(SimpleObject, Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct UserToken {
    /// User token general representation.
    pub token: String,
}

impl From<CoreUserToken> for UserToken {
    fn from(token: CoreUserToken) -> Self {
        Self { token: token.token }
    }
}

impl From<UserToken> for CoreUserToken {
    fn from(token: UserToken) -> Self {
        Self { token: token.token }
    }
}

/// User filters of the Flexible Project server.
#[derive(InputObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UserFilters {
    /// User identifier filters.
    id: Option<IdFilters>,
    /// User name filters.
    name: Option<UsernameFilters>,
}

impl From<UserFilters> for CoreUserFilters {
    fn from(filters: UserFilters) -> Self {
        Self {
            id: filters.id.map(Into::into),
            name: filters.name.map(Into::into),
            display_name: Default::default(),
            email: Default::default(),
            role: Default::default(),
        }
    }
}

/// User name filters of the Flexible Project server.
#[derive(InputObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UsernameFilters {
    /// Equality username filter.
    eq: Option<String>,
    /// Inequality username filter.
    ne: Option<String>,
    /// In username filter.
    #[graphql(name = "in")]
    r#in: Option<Vec<String>>,
    /// Not in username filter.
    nin: Option<Vec<String>>,
    /// Contains username filter.
    contains: Option<String>,
    /// Regex username filter.
    regex: Option<String>,
}

impl From<UsernameFilters> for CoreUsernameFilters {
    fn from(filters: UsernameFilters) -> Self {
        Self {
            eq: filters.eq.map(Into::into),
            ne: filters.ne.map(Into::into),
            r#in: filters.r#in.map(Into::into),
            nin: filters.nin.map(Into::into),
            contains: filters.contains.map(Into::into),
            regex: filters.regex.map(Into::into),
        }
    }
}
