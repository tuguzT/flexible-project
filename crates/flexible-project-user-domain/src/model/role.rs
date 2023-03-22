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
