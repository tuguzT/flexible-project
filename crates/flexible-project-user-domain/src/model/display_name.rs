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
pub struct DisplayNameFilters {
    /// Equality user display name filter.
    pub eq: Option<Equal<DisplayName>>,
    /// Inequality user display name filter.
    pub ne: Option<NotEqual<DisplayName>>,
    /// In user display name filter.
    pub r#in: Option<In<DisplayName>>,
    /// Not in user display name filter.
    pub nin: Option<NotIn<DisplayName>>,
    /// Regex user display name filter.
    pub regex: Option<Regex>,
}

impl Filter for DisplayNameFilters {
    type Input<'a> = &'a DisplayName
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
            && regex.satisfies(input.as_str())
    }
}
