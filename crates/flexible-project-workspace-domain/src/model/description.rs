use std::borrow::Borrow;

use derive_more::Display;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use typed_builder::TypedBuilder;

/// Description of the workspace in Markdown format.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Description(String);

impl Description {
    /// Creates new workspace description from input string.
    pub fn new(description: impl Into<String>) -> Self {
        let description = description.into();
        Self(description)
    }

    /// Extracts string slice from a workspace description.
    pub fn as_str(&self) -> &str {
        let Self(description) = self;
        description.as_str()
    }

    /// Converts workspace description into a string.
    pub fn into_inner(self) -> String {
        let Self(description) = self;
        description
    }
}

/// Filters for workspace description of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct DescriptionFilters<'a> {
    /// Equality workspace description filter.
    pub eq: Option<Equal<'a, Description>>,
    /// Inequality workspace description filter.
    pub ne: Option<NotEqual<'a, Description>>,
    /// In workspace description filter.
    pub r#in: Option<In<'a, Description>>,
    /// Not in workspace description filter.
    pub nin: Option<NotIn<'a, Description>>,
    /// Regex workspace description filter.
    pub regex: Option<Regex<'a>>,
}

impl Filter for DescriptionFilters<'_> {
    type Input = Description;

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
