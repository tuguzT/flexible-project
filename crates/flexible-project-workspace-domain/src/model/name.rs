use std::borrow::Borrow;

use derive_more::{Display, Error};
use fancy_regex::Regex as FancyRegex;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use once_cell::sync::Lazy;
use typed_builder::TypedBuilder;

/// Name of the workspace in the system with strong requirements about its content.
///
/// These requirements are:
/// - must not be empty;
/// - must not be larger than 128 characters in length;
/// - must contain at least one letter.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(String);

impl Name {
    /// Creates new workspace name from input string.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if input string does not match workspace name requirements.
    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        static REGEX: Lazy<FancyRegex> = Lazy::new(|| {
            FancyRegex::new(r"^(?=.*?\p{L}).{1,128}$").expect("regex pattern should be parsed")
        });

        let name = name.into();
        let is_valid = REGEX
            .is_match(&name)
            .expect("input name matching should be successful");
        if !is_valid {
            return Err(NameError::Invalid);
        }
        Ok(Self(name))
    }

    /// Extracts string slice from a workspace name.
    pub fn as_str(&self) -> &str {
        let Self(name) = self;
        name.as_str()
    }

    /// Converts workspace name into a string.
    pub fn into_inner(self) -> String {
        let Self(name) = self;
        name
    }
}

/// Type of error which is returned when input does not meet workspace name requirements.
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum NameError {
    /// Workspace name does not meet requirements.
    #[display(fmt = "workspace name does not meet requirements")]
    Invalid,
}

/// Filters for user name of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct NameFilters<'a> {
    /// Equality user name filter.
    pub eq: Option<Equal<&'a Name>>,
    /// Inequality user name filter.
    pub ne: Option<NotEqual<&'a Name>>,
    /// In user name filter.
    pub r#in: Option<In<&'a [Name]>>,
    /// Not in user name filter.
    pub nin: Option<NotIn<&'a [Name]>>,
    /// Regex user name filter.
    pub regex: Option<Regex<&'a str>>,
}

impl<Input> Filter<Input> for NameFilters<'_>
where
    Input: Borrow<Name>,
{
    fn satisfies(&self, input: Input) -> bool {
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
