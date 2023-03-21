//! Model of the user microservice domain layer.

use std::hash::{Hash, Hasher};

use fp_core::{
    filter::{Equal, Filter, In, NotEqual, NotIn, Regex},
    id::{Id, IdFilters},
};
use typed_builder::TypedBuilder;

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

/// Filters for user identifiers of the backend.
pub type UserIdFilters = IdFilters<User>;

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
    type Input<'a> = &'a User
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self {
            id,
            name,
            display_name,
            role,
            email,
        } = self;
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

/// User name filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct NameFilters {
    /// Equality user name filter.
    pub eq: Option<Equal<String>>,
    /// Inequality user name filter.
    pub ne: Option<NotEqual<String>>,
    /// In user name filter.
    pub r#in: Option<In<String>>,
    /// Not in user name filter.
    pub nin: Option<NotIn<String>>,
    /// Regex user name filter.
    pub regex: Option<Regex>,
}

impl Filter for NameFilters {
    type Input<'a> = &'a String
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;
        eq.satisfies(input)
            && ne.satisfies(input)
            && r#in.satisfies(input)
            && nin.satisfies(input)
            && regex.satisfies(input)
    }
}

/// User display name filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct DisplayNameFilters {
    /// Equality user display name filter.
    pub eq: Option<Equal<String>>,
    /// Inequality user display name filter.
    pub ne: Option<NotEqual<String>>,
    /// In user display name filter.
    pub r#in: Option<In<String>>,
    /// Not in user display name filter.
    pub nin: Option<NotIn<String>>,
    /// Regex user display name filter.
    pub regex: Option<Regex>,
}

impl Filter for DisplayNameFilters {
    type Input<'a> = &'a String
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;
        eq.satisfies(input)
            && ne.satisfies(input)
            && r#in.satisfies(input)
            && nin.satisfies(input)
            && regex.satisfies(input)
    }
}

/// User role filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleFilters {
    /// Equality user role filter.
    pub eq: Option<Equal<Role>>,
    /// Inequality user role filter.
    pub ne: Option<NotEqual<Role>>,
    /// In user role filter.
    pub r#in: Option<In<Role>>,
    /// Not in user role filter.
    pub nin: Option<NotIn<Role>>,
}

impl Filter for RoleFilters {
    type Input<'a> = &'a Role
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self { eq, ne, r#in, nin } = self;
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}

/// User email filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct EmailFilters {
    /// Equality user email filter.
    pub eq: Option<Equal<String>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<String>>,
    /// In user email filter.
    pub r#in: Option<In<String>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<String>>,
    /// Regex user email filter.
    pub regex: Option<Regex>,
}

impl Filter for EmailFilters {
    type Input<'a> = &'a String
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;
        eq.satisfies(input)
            && ne.satisfies(input)
            && r#in.satisfies(input)
            && nin.satisfies(input)
            && regex.satisfies(input)
    }
}
