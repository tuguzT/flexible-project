use std::borrow::Cow;

use derive_more::Display;
use fp_user_domain::model::{Name as DomainName, NameError, NameFilters as DomainNameFilters};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn, Regex};

/// Serializable [name](DomainName) of the user.
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Name(String);

impl From<DomainName> for Name {
    fn from(name: DomainName) -> Self {
        let name = name.into_inner();
        Self(name)
    }
}

impl TryFrom<Name> for DomainName {
    type Error = NameError;

    fn try_from(name: Name) -> Result<Self, Self::Error> {
        let Name(name) = name;
        DomainName::new(name)
    }
}

/// Filters for user name of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct NameFilters {
    /// Equality user name filter.
    pub eq: Option<Equal<Name>>,
    /// Inequality user name filter.
    pub ne: Option<NotEqual<Name>>,
    /// In user name filter.
    pub r#in: Option<In<Vec<Name>>>,
    /// Not in user name filter.
    pub nin: Option<NotIn<Vec<Name>>>,
    /// Regex user name filter.
    pub regex: Option<Regex<String>>,
}

impl From<DomainNameFilters<'_>> for NameFilters {
    fn from(filters: DomainNameFilters<'_>) -> Self {
        let DomainNameFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        Self {
            eq: eq.map(|name| Equal(name.0.into_owned().into())),
            ne: ne.map(|name| NotEqual(name.0.into_owned().into())),
            r#in: r#in.map(|r#in| In(r#in.0.iter().cloned().map(Into::into).collect())),
            nin: nin.map(|r#in| NotIn(r#in.0.iter().cloned().map(Into::into).collect())),
            regex: regex.map(|regex| Regex(regex.0.into_owned())),
        }
    }
}

impl TryFrom<NameFilters> for DomainNameFilters<'_> {
    type Error = NameError;

    fn try_from(filters: NameFilters) -> Result<Self, Self::Error> {
        let NameFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        let eq = eq
            .map(|Equal(name)| {
                let name = name.try_into()?;
                let filter = Equal(Cow::Owned(name)).into();
                Ok(filter)
            })
            .transpose()?;
        let ne = ne
            .map(|NotEqual(name)| {
                let name = name.try_into()?;
                let filter = NotEqual(Cow::Owned(name)).into();
                Ok(filter)
            })
            .transpose()?;
        let r#in = r#in
            .map(|In(names)| {
                let names = names
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = In(names.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let nin = nin
            .map(|NotIn(names)| {
                let names = names
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = NotIn(names.into()).into();
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
