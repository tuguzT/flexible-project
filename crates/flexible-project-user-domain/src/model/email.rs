use std::borrow::{Borrow, Cow};

use derive_more::{Display, Error};
use email_address::EmailAddress;
use fp_filter::{Equal, Filter, In, NotEqual, NotIn, Regex};
use typed_builder::TypedBuilder;

/// Email of the user in the system with strong requirements about its content.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Email(String);

impl Email {
    /// Creates new user email from input string.
    ///
    /// # Errors
    ///
    /// This function will return an error
    /// if input string does not match user email requirements.
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

/// Filters for user email of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct EmailFilters<'a> {
    /// Equality user email filter.
    pub eq: Option<Equal<Cow<'a, Email>>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<Cow<'a, Email>>>,
    /// In user email filter.
    pub r#in: Option<In<Cow<'a, [Email]>>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<Cow<'a, [Email]>>>,
    /// Regex user email filter.
    pub regex: Option<Regex<Cow<'a, str>>>,
}

impl<Input> Filter<Input> for EmailFilters<'_>
where
    Input: Borrow<Email>,
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
            && r#in.as_ref().map(In::as_deref).satisfies(input)
            && nin.as_ref().map(NotIn::as_deref).satisfies(input)
            && regex.satisfies(input.as_str())
    }
}

/// Filters for optional user email of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct OptionEmailFilters<'a> {
    /// Equality user email filter.
    pub eq: Option<Equal<Cow<'a, Option<Email>>>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<Cow<'a, Option<Email>>>>,
    /// In user email filter.
    pub r#in: Option<In<Cow<'a, [Option<Email>]>>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<Cow<'a, [Option<Email>]>>>,
    /// Regex user email filter.
    pub regex: Option<Regex<Cow<'a, str>>>,
}

impl<Input> Filter<Input> for OptionEmailFilters<'_>
where
    Input: Borrow<Option<Email>>,
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
            && r#in.as_ref().map(In::as_deref).satisfies(input)
            && nin.as_ref().map(NotIn::as_deref).satisfies(input)
            && input
                .as_ref()
                .map(|input| regex.satisfies(input.as_str()))
                .unwrap_or(true)
    }
}

#[cfg(test)]
mod test {
    use super::{Email, EmailError};

    #[test]
    fn valid_ones() {
        let Email(_) = Email::new("example@email.com").unwrap();
        let Email(_) = Email::new("example.firstname-lastname@email.com").unwrap();
        let Email(_) = Email::new("timurka.tugushev@gmail.com").unwrap();
        let Email(_) = Email::new("tugushev.t.r@edu.mirea.ru").unwrap();
        let Email(_) = Email::new("nik.3989@mail.ru").unwrap();
    }

    #[test]
    fn invalid() {
        let _: EmailError = Email::new("John Doe <example@email.com>").unwrap_err();
        let _: EmailError = Email::new("plaintext").unwrap_err();
        let _: EmailError = Email::new("@email.com").unwrap_err();
        let _: EmailError = Email::new(r#"is"especially"not\allowed@email.com"#).unwrap_err();
    }
}
