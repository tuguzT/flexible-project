use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_core::filter::Filter;
use typed_builder::TypedBuilder;

use super::{
    display_name::{DisplayName, DisplayNameFilters},
    email::{Email, EmailFilters},
    id::{UserId, UserIdFilters},
    name::{Name, NameFilters},
    role::{Role, RoleFilters},
};

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
    pub name: Name,
    /// Display name of the user.
    pub display_name: DisplayName,
    /// Role of the user.
    pub role: Role,
    /// Unique email of the user, if exists.
    pub email: Option<Email>,
}

/// Filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserFilters {
    /// User identifier filters.
    pub id: Option<UserIdFilters>,
    /// User name filters.
    pub name: Option<NameFilters>,
    /// User display name filters.
    pub display_name: Option<DisplayNameFilters>,
    /// User role filters.
    pub role: Option<RoleFilters>,
    /// User email filters.
    pub email: Option<EmailFilters>,
}

impl Filter for UserFilters {
    type Input = User;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self {
            id,
            name,
            display_name,
            role,
            email,
        } = self;
        let input = input.borrow();
        id.satisfies(&input.id)
            && name.satisfies(&input.data.name)
            && display_name.satisfies(&input.data.display_name)
            && role.satisfies(&input.data.role)
            && input
                .data
                .email
                .as_ref()
                .map(|input| email.satisfies(input))
                .unwrap_or(true)
    }
}
