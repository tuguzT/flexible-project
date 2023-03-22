use std::borrow::Borrow;

use derive_more::{Display, Error, From};
use fancy_regex::Regex as FancyRegex;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use once_cell::sync::Lazy;
use typed_builder::TypedBuilder;

/// User name with strong requirements about its content.
///
/// These requirements are:
/// - must be from 4 to 32 characters in length;
/// - must contain latin or `-`, `_`, `.` characters;
/// - must not start or end with `-`, `_`, `.` characters;
/// - `-`, `_`, `.` characters can't be next to each other;
/// - `-`, `_`, `.` characters can't be used multiple times in a row.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name(String);

impl Name {
    /// Creates new user name from input string.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if input string does not match user name requirements.
    pub fn new(name: String) -> Result<Self, NameError> {
        static REGEX: Lazy<FancyRegex> = Lazy::new(|| {
            FancyRegex::new(r"^(?=.{4,32}$)(?![-_.])(?!.*[-_.]{2})[a-zA-Z\d\-_.]+(?<![-_.])$")
                .expect("regex pattern should be parsed")
        });

        let is_valid = REGEX
            .is_match(&name)
            .expect("input name matching should be successful");
        if !is_valid {
            return Err(NameError::Invalid);
        }
        Ok(Self(name))
    }

    /// Extracts string slice from a user name.
    pub fn as_str(&self) -> &str {
        let Self(name) = self;
        name.as_str()
    }

    /// Converts user name into a string.
    pub fn into_inner(self) -> String {
        let Self(name) = self;
        name
    }
}

/// Type of error which is returned when input does not meet user name requirements.
#[derive(Debug, Display, Clone, Copy, From, Error)]
pub enum NameError {
    /// User name does not meet requirements.
    #[display(fmt = "user name does not meet requirements")]
    Invalid,
}

/// User name filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct NameFilters {
    /// Equality user name filter.
    pub eq: Option<Equal<Name>>,
    /// Inequality user name filter.
    pub ne: Option<NotEqual<Name>>,
    /// In user name filter.
    pub r#in: Option<In<Name>>,
    /// Not in user name filter.
    pub nin: Option<NotIn<Name>>,
    /// Regex user name filter.
    pub regex: Option<Regex>,
}

impl Filter for NameFilters {
    type Input = Name;

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
