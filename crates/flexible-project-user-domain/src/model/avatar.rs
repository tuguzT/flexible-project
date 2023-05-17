use std::borrow::Borrow;

use derive_more::{Display, Error};
use fp_filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use typed_builder::TypedBuilder;
use url::Url;

/// User avatar URL of the user in the system.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Avatar(Url);

impl Avatar {
    /// Creates new user avatar URL from input string.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if input string does not match user avatar URL requirements.
    pub fn new(url: impl Into<String>) -> Result<Self, AvatarError> {
        let url = url.into();
        let Ok(url) = url.parse() else {
            return Err(AvatarError::Invalid);
        };
        Ok(Self(url))
    }

    /// Extracts string slice from a user avatar URL.
    pub fn as_str(&self) -> &str {
        let Self(url) = self;
        url.as_str()
    }

    /// Converts user avatar URL into a string.
    pub fn into_inner(self) -> String {
        let Self(url) = self;
        url.to_string()
    }
}

/// Type of error which is returned when input does not meet user avatar URL requirements.
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum AvatarError {
    /// User avatar URL does not meet requirements.
    #[display(fmt = "user avatar URL does not meet requirements")]
    Invalid,
}

/// Filters for user avatar URL of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct AvatarFilters<'a> {
    /// Equality user avatar filter.
    pub eq: Option<Equal<&'a Avatar>>,
    /// Inequality user avatar filter.
    pub ne: Option<NotEqual<&'a Avatar>>,
    /// In user avatar filter.
    pub r#in: Option<In<&'a [Avatar]>>,
    /// Not in user avatar filter.
    pub nin: Option<NotIn<&'a [Avatar]>>,
    /// Regex user avatar filter.
    pub regex: Option<Regex<&'a str>>,
}

impl<Input> Filter<Input> for AvatarFilters<'_>
where
    Input: Borrow<Avatar>,
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

/// Filters for optional user avatar URL of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct OptionAvatarFilters<'a> {
    /// Equality user avatar filter.
    pub eq: Option<Equal<&'a Option<Avatar>>>,
    /// Inequality user avatar filter.
    pub ne: Option<NotEqual<&'a Option<Avatar>>>,
    /// In user avatar filter.
    pub r#in: Option<In<&'a [Option<Avatar>]>>,
    /// Not in user avatar filter.
    pub nin: Option<NotIn<&'a [Option<Avatar>]>>,
    /// Regex user avatar filter.
    pub regex: Option<Regex<&'a str>>,
}

impl<Input> Filter<Input> for OptionAvatarFilters<'_>
where
    Input: Borrow<Option<Avatar>>,
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
            && input
                .as_ref()
                .map(|input| regex.satisfies(input.as_str()))
                .unwrap_or(true)
    }
}

#[cfg(test)]
mod test {
    use super::{Avatar, AvatarError};

    #[test]
    fn valid_ones() {
        let Avatar(_) = Avatar::new("https://vk.com/im").unwrap();
        let Avatar(_) = Avatar::new("https://github.com/rust-lang/rust/issues").unwrap();
        let Avatar(_) = Avatar::new("https://docs.rs/url/latest/url/index.html").unwrap();
    }

    #[test]
    fn invalid() {
        let _: AvatarError = Avatar::new("http://[:::1]").unwrap_err();
        let _: AvatarError = Avatar::new("../main.css").unwrap_err();
    }
}
