use derive_more::Display;
use fp_user_domain::model::{Email as DomainEmail, EmailError};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn, Regex};

/// Serializable [email](DomainEmail) of the user.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl From<DomainEmail> for Email {
    fn from(email: DomainEmail) -> Self {
        let email = email.into_inner();
        Self(email)
    }
}

impl TryFrom<Email> for DomainEmail {
    type Error = EmailError;

    fn try_from(email: Email) -> Result<Self, Self::Error> {
        let Email(email) = email;
        DomainEmail::new(email)
    }
}

/// Filters for user email of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct EmailFilters {
    /// Equality user email filter.
    pub eq: Option<Equal<Email>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<Email>>,
    /// In user email filter.
    pub r#in: Option<In<Vec<Email>>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<Vec<Email>>>,
    /// Regex user email filter.
    pub regex: Option<Regex<String>>,
}

/// Filters for optional user email of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct OptionEmailFilters {
    /// Equality user email filter.
    pub eq: Option<Equal<Option<Email>>>,
    /// Inequality user email filter.
    pub ne: Option<NotEqual<Option<Email>>>,
    /// In user email filter.
    pub r#in: Option<In<Vec<Option<Email>>>>,
    /// Not in user email filter.
    pub nin: Option<NotIn<Vec<Option<Email>>>>,
    /// Regex user email filter.
    pub regex: Option<Regex<String>>,
}
