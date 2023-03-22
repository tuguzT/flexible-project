use std::borrow::Borrow;

use derive_more::Display;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn};
use typed_builder::TypedBuilder;

/// Role of the user in the system.
#[derive(Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
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

/// User role filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleFilters<'a> {
    /// Equality user role filter.
    pub eq: Option<Equal<'a, Role>>,
    /// Inequality user role filter.
    pub ne: Option<NotEqual<'a, Role>>,
    /// In user role filter.
    pub r#in: Option<In<'a, Role>>,
    /// Not in user role filter.
    pub nin: Option<NotIn<'a, Role>>,
}

impl Filter for RoleFilters<'_> {
    type Input = Role;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self { eq, ne, r#in, nin } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}
