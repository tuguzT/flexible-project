//! User definitions and utilities for the Flexible Project system model.

#![allow(missing_docs)]

use derive_more::{Display, From, IsVariant, Unwrap};
use typed_builder::TypedBuilder;

use crate::model::filter::{Contains, Equal, In, NotEqual, NotIn, Regex};
use crate::model::id::{Id, IdFilters};

/// Type of [user](User) identifier.
pub type UserId = Id<User>;

/// Data of user of the Flexible Project system.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    /// Identifier of the user.
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

/// User access token which is required to access non-public system resources.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct UserToken {
    /// User token general representation.
    pub token: String,
}

/// Claims stored securely inside of the [user token](UserToken).
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct UserTokenClaims {
    /// Identifier of the user.
    pub id: Id<User>,
}

/// Filters to be applied on user search process.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserFilters {
    /// User identifier filters.
    pub id: Option<IdFilters<User>>,
    /// User name filters.
    pub name: Option<UsernameFilters>,
    /// User display name filters.
    pub display_name: Option<UserDisplayNameFilters>,
    /// User email filters.
    pub email: Option<UserEmailFilters>,
    /// User role filters.
    pub role: Option<UserRoleFilters>,
}

/// User name filters to be applied on user search process.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UsernameFilters {
    /// Equality username filter.
    pub eq: Option<Equal<String>>,
    /// Inequality username filter.
    pub ne: Option<NotEqual<String>>,
    /// In username filter.
    pub r#in: Option<In<String>>,
    /// Not in username filter.
    pub nin: Option<NotIn<String>>,
    /// Contains username filter.
    pub contains: Option<Contains>,
    /// Regex username filter.
    pub regex: Option<Regex>,
}

/// User display name filters to be applied on user search process.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserDisplayNameFilters {
    /// Equality user display name filter.
    pub eq: Option<Equal<String>>,
    /// Inequality user display name filter.
    pub ne: Option<NotEqual<String>>,
    /// In user display name filter.
    pub r#in: Option<In<String>>,
    /// Not in user display name filter.
    pub nin: Option<NotIn<String>>,
    /// Contains user display name filter.
    pub contains: Option<Contains>,
    /// Regex user display name filter.
    pub regex: Option<Regex>,
}

/// User email filters to be applied on user search process.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserEmailFilters {
    /// Equality user email filter.
    pub eq: Option<Equal<String>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<String>>,
    /// In user email filter.
    pub r#in: Option<In<String>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<String>>,
    /// Contains user email filter.
    pub contains: Option<Contains>,
    /// Regex user email filter.
    pub regex: Option<Regex>,
}

/// User role filters to be applied on user search process.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserRoleFilters {
    /// Equality user role filter.
    pub eq: Option<Equal<UserRole>>,
    /// Inequality user role filter.
    pub ne: Option<NotEqual<UserRole>>,
    /// In user role filter.
    pub r#in: Option<In<UserRole>>,
    /// Not in user role filter.
    pub nin: Option<NotIn<UserRole>>,
}
