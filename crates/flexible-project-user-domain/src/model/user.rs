use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_filter::Filter;
use typed_builder::TypedBuilder;

use super::{
    avatar::{Avatar, OptionAvatarFilters},
    display_name::{DisplayName, DisplayNameFilters},
    email::{Email, OptionEmailFilters},
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
    /// Unique email of the user, if present.
    pub email: Option<Email>,
    /// Avatar URL of the user, if present.
    pub avatar: Option<Avatar>,
}

/// Filters for user of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserFilters<'a> {
    /// User identifier filters.
    pub id: Option<UserIdFilters<'a>>,
    /// User data filters.
    pub data: Option<UserDataFilters<'a>>,
}

impl<Input> Filter<Input> for UserFilters<'_>
where
    Input: Borrow<User>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            id: id_filter,
            data: data_filter,
        } = self;
        let User { id, data } = input.borrow();
        id_filter.satisfies(id) && data_filter.satisfies(data)
    }
}

/// Filters for user data of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserDataFilters<'a> {
    /// User name filters.
    pub name: Option<NameFilters<'a>>,
    /// User display name filters.
    pub display_name: Option<DisplayNameFilters<'a>>,
    /// User role filters.
    pub role: Option<RoleFilters<'a>>,
    /// User email filters.
    pub email: Option<OptionEmailFilters<'a>>,
    /// User avatar filters.
    pub avatar: Option<OptionAvatarFilters<'a>>,
}

impl<Input> Filter<Input> for UserDataFilters<'_>
where
    Input: Borrow<UserData>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            name: name_filter,
            display_name: display_name_filter,
            role: role_filter,
            email: email_filter,
            avatar: avatar_filter,
        } = self;
        let UserData {
            name,
            display_name,
            role,
            email,
            avatar,
        } = input.borrow();
        name_filter.satisfies(name)
            && display_name_filter.satisfies(display_name)
            && role_filter.satisfies(role)
            && email_filter.satisfies(email)
            && avatar_filter.satisfies(avatar)
    }
}
