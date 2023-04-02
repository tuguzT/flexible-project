use std::borrow::Borrow;

use derive_more::{Display, Error};
use fancy_regex::Regex as FancyRegex;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use once_cell::sync::Lazy;
use typed_builder::TypedBuilder;

/// Display name of the user in the system with strong requirements about its content.
///
/// These requirements are:
/// - must not be empty;
/// - must not be larger than 128 characters in length;
/// - must contain at least one letter.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayName(String);

impl DisplayName {
    /// Creates new user display name from input string.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if input string does not match user display name requirements.
    pub fn new(display_name: impl Into<String>) -> Result<Self, DisplayNameError> {
        static REGEX: Lazy<FancyRegex> = Lazy::new(|| {
            FancyRegex::new(r"^(?=.*?\p{L}).{1,128}$").expect("regex pattern should be parsed")
        });

        let display_name = display_name.into();
        let is_valid = REGEX
            .is_match(&display_name)
            .expect("input name matching should be successful");
        if !is_valid {
            return Err(DisplayNameError::Invalid);
        }
        Ok(Self(display_name))
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

/// Type of error which is returned when input does not meet user display name requirements.
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum DisplayNameError {
    /// User display name does not meet requirements.
    #[display(fmt = "user display name does not meet requirements")]
    Invalid,
}

/// Filters for user display name of the backend.
#[derive(Debug, Clone, Copy, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct DisplayNameFilters<'a> {
    /// Equality user display name filter.
    pub eq: Option<Equal<&'a DisplayName>>,
    /// Inequality user display name filter.
    pub ne: Option<NotEqual<&'a DisplayName>>,
    /// In user display name filter.
    pub r#in: Option<In<&'a [DisplayName]>>,
    /// Not in user display name filter.
    pub nin: Option<NotIn<&'a [DisplayName]>>,
    /// Regex user display name filter.
    pub regex: Option<Regex<&'a str>>,
}

impl<Input> Filter<Input> for DisplayNameFilters<'_>
where
    Input: Borrow<DisplayName>,
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

#[cfg(test)]
mod test {
    use super::{DisplayName, DisplayNameError};

    #[test]
    fn valid_ones() {
        let DisplayName(_) = DisplayName::new("Тимур Тугушев").unwrap();
        let DisplayName(_) = DisplayName::new("Timur_Tugushev").unwrap();
        let DisplayName(_) = DisplayName::new("__ёжик__").unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        let DisplayName(_) = DisplayName::new("").unwrap();
    }

    #[test]
    fn no_letters() {
        let _: DisplayNameError = DisplayName::new("__1__").unwrap_err();
        let _: DisplayNameError = DisplayName::new("0123456789").unwrap_err();
        let _: DisplayNameError = DisplayName::new("&%!@@(*&@$@*").unwrap_err();
    }
}
