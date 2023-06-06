use std::borrow::{Borrow, Cow};

use derive_more::Display;
use fp_filter::{Equal, Filter, In, NotEqual, NotIn};
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

/// Filters for user role of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct RoleFilters<'a> {
    /// Equality user role filter.
    pub eq: Option<Equal<Cow<'a, Role>>>,
    /// Inequality user role filter.
    pub ne: Option<NotEqual<Cow<'a, Role>>>,
    /// In user role filter.
    pub r#in: Option<In<Cow<'a, [Role]>>>,
    /// Not in user role filter.
    pub nin: Option<NotIn<Cow<'a, [Role]>>>,
}

impl<Input> Filter<Input> for RoleFilters<'_>
where
    Input: Borrow<Role>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { eq, ne, r#in, nin } = self;
        let input = input.borrow();
        eq.satisfies(Cow::Borrowed(input))
            && ne.satisfies(Cow::Borrowed(input))
            && r#in.as_ref().map(In::as_deref).satisfies(input)
            && nin.as_ref().map(NotIn::as_deref).satisfies(input)
    }
}
