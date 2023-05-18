use std::borrow::{Borrow, Cow};

use derive_more::{Display, Error};
use fancy_regex::Regex as FancyRegex;
use fp_filter::{CowSlice, Equal, Filter, In, NotEqual, NotIn, Regex};
use once_cell::sync::Lazy;
use typed_builder::TypedBuilder;

/// Name of the user in the system with strong requirements about its content.
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
    pub fn new(name: impl Into<String>) -> Result<Self, NameError> {
        static REGEX: Lazy<FancyRegex> = Lazy::new(|| {
            FancyRegex::new(r"^(?=.{4,32}$)(?![-_.])(?!.*[-_.]{2})[a-zA-Z\d\-_.]+(?<![-_.])$")
                .expect("regex pattern should be parsed")
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
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum NameError {
    /// User name does not meet requirements.
    #[display(fmt = "user name does not meet requirements")]
    Invalid,
}

/// Filters for user name of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct NameFilters<'a> {
    /// Equality user name filter.
    pub eq: Option<Equal<Cow<'a, Name>>>,
    /// Inequality user name filter.
    pub ne: Option<NotEqual<Cow<'a, Name>>>,
    /// In user name filter.
    pub r#in: Option<In<CowSlice<'a, Name>>>,
    /// Not in user name filter.
    pub nin: Option<NotIn<CowSlice<'a, Name>>>,
    /// Regex user name filter.
    pub regex: Option<Regex<Cow<'a, str>>>,
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
        eq.satisfies(Cow::Borrowed(input))
            && ne.satisfies(Cow::Borrowed(input))
            && r#in.satisfies(input)
            && nin.satisfies(input)
            && regex.satisfies(input.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::{Name, NameError};

    #[test]
    fn valid_ones() {
        let Name(_) = Name::new("tuguzT").unwrap();
        let Name(_) = Name::new("lower_snake_case").unwrap();
        let Name(_) = Name::new("SCREAMING_SNAKE_CASE").unwrap();

        let Name(_) = Name::new("tugushev_timur_q").unwrap();
        let Name(_) = Name::new("any-harmony").unwrap();
        let Name(_) = Name::new("thirty.two.characters.supported").unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        let Name(_) = Name::new("").unwrap();
    }

    #[test]
    #[should_panic]
    fn too_short() {
        let Name(_) = Name::new("hey").unwrap();
    }

    #[test]
    #[should_panic]
    fn too_long() {
        let Name(_) = Name::new("too-many-characters-in-one-username").unwrap();
    }

    #[test]
    fn start_or_end_special() {
        let _: NameError = Name::new("_tugushev_timur").unwrap_err();
        let _: NameError = Name::new("tugushev_timur_").unwrap_err();
        let _: NameError = Name::new(".some.username").unwrap_err();
        let _: NameError = Name::new("another-username-").unwrap_err();
    }

    #[test]
    fn too_many_special_in_row() {
        let _: NameError = Name::new("tugushev__timur").unwrap_err();
        let _: NameError = Name::new("Tugushev._Timur").unwrap_err();
        let _: NameError = Name::new("some--username").unwrap_err();
        let _: NameError = Name::new("another..username").unwrap_err();
    }

    #[test]
    fn non_latin() {
        let _: NameError = Name::new("привет_мир").unwrap_err();
        let _: NameError = Name::new("асу").unwrap_err();
    }
}
