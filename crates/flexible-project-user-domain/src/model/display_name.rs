use std::borrow::Borrow;

use derive_more::Display;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use typed_builder::TypedBuilder;

/// Display name of the user in the system.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayName(String);

impl DisplayName {
    /// Creates new user display name from input string.
    pub fn new(display_name: String) -> Self {
        Self(display_name)
    }

    /// Extracts string slice from a user display name.
    pub fn as_str(&self) -> &str {
        let Self(display_name) = self;
        display_name.as_str()
    }

    /// Converts user display name into a string.
    pub fn into_inner(self) -> String {
        let Self(display_name) = self;
        display_name
    }
}

/// User display name filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct DisplayNameFilters<'a> {
    /// Equality user display name filter.
    pub eq: Option<Equal<'a, DisplayName>>,
    /// Inequality user display name filter.
    pub ne: Option<NotEqual<'a, DisplayName>>,
    /// In user display name filter.
    pub r#in: Option<In<'a, DisplayName>>,
    /// Not in user display name filter.
    pub nin: Option<NotIn<'a, DisplayName>>,
    /// Regex user display name filter.
    pub regex: Option<Regex<'a>>,
}

impl Filter for DisplayNameFilters<'_> {
    type Input = DisplayName;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = self;
        let input = input.borrow();
        eq.satisfies(input)
            && ne.satisfies(input)
            && r#in.satisfies(input)
            && nin.satisfies(input)
            && regex.satisfies(input.as_str())
    }
}
