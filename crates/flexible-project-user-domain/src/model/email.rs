use std::borrow::Borrow;

use derive_more::Display;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use typed_builder::TypedBuilder;

/// Email of the user in the system.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Email(String);

impl Email {
    /// Creates new user email from input string.
    pub fn new(email: String) -> Self {
        Self(email)
    }

    /// Extracts string slice from a user email.
    pub fn as_str(&self) -> &str {
        let Self(email) = self;
        email.as_str()
    }

    /// Converts user email into a string.
    pub fn into_inner(self) -> String {
        let Self(email) = self;
        email
    }
}

/// User email filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct EmailFilters<'a> {
    /// Equality user email filter.
    pub eq: Option<Equal<'a, Email>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<'a, Email>>,
    /// In user email filter.
    pub r#in: Option<In<'a, Email>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<'a, Email>>,
    /// Regex user email filter.
    pub regex: Option<Regex<'a>>,
}

impl Filter for EmailFilters<'_> {
    type Input = Email;

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