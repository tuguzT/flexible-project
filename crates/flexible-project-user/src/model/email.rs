use std::borrow::Cow;

use derive_more::Display;
use fp_user_domain::model::{
    Email as DomainEmail, EmailError, EmailFilters as DomainEmailFilters,
    OptionEmailFilters as DomainOptionEmailFilters,
};
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

impl From<DomainEmailFilters<'_>> for EmailFilters {
    fn from(filters: DomainEmailFilters<'_>) -> Self {
        let DomainEmailFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        Self {
            eq: eq.map(|email| Equal(email.0.into_owned().into())),
            ne: ne.map(|email| NotEqual(email.0.into_owned().into())),
            r#in: r#in.map(|r#in| {
                let cow_slice = r#in.0;
                In(cow_slice.0.iter().cloned().map(Into::into).collect())
            }),
            nin: nin.map(|r#in| {
                let cow_slice = r#in.0;
                NotIn(cow_slice.0.iter().cloned().map(Into::into).collect())
            }),
            regex: regex.map(|regex| Regex(regex.0.into_owned())),
        }
    }
}

impl TryFrom<EmailFilters> for DomainEmailFilters<'_> {
    type Error = EmailError;

    fn try_from(filters: EmailFilters) -> Result<Self, Self::Error> {
        let EmailFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        let eq = eq
            .map(|Equal(email)| {
                let email = email.try_into()?;
                let filter = Equal(Cow::Owned(email)).into();
                Ok(filter)
            })
            .transpose()?;
        let ne = ne
            .map(|NotEqual(email)| {
                let email = email.try_into()?;
                let filter = NotEqual(Cow::Owned(email)).into();
                Ok(filter)
            })
            .transpose()?;
        let r#in = r#in
            .map(|In(emails)| {
                let emails = emails
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = In(emails.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let nin = nin
            .map(|NotIn(emails)| {
                let emails = emails
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = NotIn(emails.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let regex = regex.map(|Regex(regex)| Regex(Cow::Owned(regex)).into());
        let filters = Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        };
        Ok(filters)
    }
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

impl From<DomainOptionEmailFilters<'_>> for OptionEmailFilters {
    fn from(filters: DomainOptionEmailFilters<'_>) -> Self {
        let DomainOptionEmailFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        Self {
            eq: eq.map(|email| Equal(email.0.into_owned().map(Into::into))),
            ne: ne.map(|email| NotEqual(email.0.into_owned().map(Into::into))),
            r#in: r#in.map(|r#in| {
                let cow_slice = r#in.0;
                let iter = cow_slice.0.iter();
                In(iter.cloned().map(|email| email.map(Into::into)).collect())
            }),
            nin: nin.map(|r#in| {
                let cow_slice = r#in.0;
                let iter = cow_slice.0.iter();
                NotIn(iter.cloned().map(|email| email.map(Into::into)).collect())
            }),
            regex: regex.map(|regex| Regex(regex.0.into_owned())),
        }
    }
}

impl TryFrom<OptionEmailFilters> for DomainOptionEmailFilters<'_> {
    type Error = EmailError;

    fn try_from(filters: OptionEmailFilters) -> Result<Self, Self::Error> {
        let OptionEmailFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        let eq = eq
            .map(|Equal(email)| {
                let email = email.map(TryInto::try_into).transpose()?;
                let filter = Equal(Cow::Owned(email)).into();
                Ok(filter)
            })
            .transpose()?;
        let ne = ne
            .map(|NotEqual(email)| {
                let email = email.map(TryInto::try_into).transpose()?;
                let filter = NotEqual(Cow::Owned(email)).into();
                Ok(filter)
            })
            .transpose()?;
        let r#in = r#in
            .map(|In(emails)| {
                let emails = emails
                    .into_iter()
                    .map(|email| email.map(TryInto::try_into).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = In(emails.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let nin = nin
            .map(|NotIn(emails)| {
                let emails = emails
                    .into_iter()
                    .map(|email| email.map(TryInto::try_into).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = NotIn(emails.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let regex = regex.map(|Regex(regex)| Regex(Cow::Owned(regex)).into());
        let filters = Self {
            eq,
            ne,
            r#in,
            nin,
            regex,
        };
        Ok(filters)
    }
}
