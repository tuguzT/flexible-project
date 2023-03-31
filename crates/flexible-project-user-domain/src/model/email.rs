use std::borrow::Borrow;

use derive_more::{Display, Error};
use email_address::EmailAddress;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use typed_builder::TypedBuilder;

/// Email of the user in the system with strong requirements about its content.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Email(String);

impl Email {
    /// Creates new user email from input string.
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into();
        let is_valid = EmailAddress::is_valid(&email);
        if !is_valid {
            return Err(EmailError::Invalid);
        }
        Ok(Self(email))
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

/// Type of error which is returned when input does not meet user email requirements.
#[derive(Debug, Display, Clone, Copy, Error)]
pub enum EmailError {
    /// User email does not meet requirements.
    #[display(fmt = "user email does not meet requirements")]
    Invalid,
}

/// User email filters to be applied on user search process.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct EmailFilters<'a> {
    /// Equality user email filter.
    pub eq: Option<Equal<'a, Option<Email>>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<'a, Option<Email>>>,
    /// In user email filter.
    pub r#in: Option<In<'a, Option<Email>>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<'a, Option<Email>>>,
    /// Regex user email filter.
    pub regex: Option<Regex<'a>>,
}

impl Filter for EmailFilters<'_> {
    type Input = Option<Email>;

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
            && input
                .as_ref()
                .map(|input| regex.satisfies(input.as_str()))
                .unwrap_or(true)
    }
}
