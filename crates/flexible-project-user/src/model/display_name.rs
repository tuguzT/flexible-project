use std::borrow::Cow;

use derive_more::Display;
use fp_user_domain::model::{
    DisplayName as DomainDisplayName, DisplayNameError,
    DisplayNameFilters as DomainDisplayNameFilters,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn, Regex};

/// Serializable [display name](DomainDisplayName) of the user.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DisplayName(String);

impl From<DomainDisplayName> for DisplayName {
    fn from(display_name: DomainDisplayName) -> Self {
        let display_name = display_name.into_inner();
        Self(display_name)
    }
}

impl TryFrom<DisplayName> for DomainDisplayName {
    type Error = DisplayNameError;

    fn try_from(display_name: DisplayName) -> Result<Self, Self::Error> {
        let DisplayName(display_name) = display_name;
        DomainDisplayName::new(display_name)
    }
}

/// Filters for user display name of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct DisplayNameFilters {
    /// Equality user display name filter.
    pub eq: Option<Equal<DisplayName>>,
    /// Inequality user display name filter.
    pub ne: Option<NotEqual<DisplayName>>,
    /// In user display name filter.
    pub r#in: Option<In<Vec<DisplayName>>>,
    /// Not in user display name filter.
    pub nin: Option<NotIn<Vec<DisplayName>>>,
    /// Regex user display name filter.
    pub regex: Option<Regex<String>>,
}

impl From<DomainDisplayNameFilters<'_>> for DisplayNameFilters {
    fn from(filters: DomainDisplayNameFilters<'_>) -> Self {
        let DomainDisplayNameFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        Self {
            eq: eq.map(|display_name| Equal(display_name.0.into_owned().into())),
            ne: ne.map(|display_name| NotEqual(display_name.0.into_owned().into())),
            r#in: r#in.map(|r#in| In(r#in.0.iter().cloned().map(Into::into).collect())),
            nin: nin.map(|r#in| NotIn(r#in.0.iter().cloned().map(Into::into).collect())),
            regex: regex.map(|regex| Regex(regex.0.into_owned())),
        }
    }
}

impl TryFrom<DisplayNameFilters> for DomainDisplayNameFilters<'_> {
    type Error = DisplayNameError;

    fn try_from(filters: DisplayNameFilters) -> Result<Self, Self::Error> {
        let DisplayNameFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        let eq = eq
            .map(|Equal(display_name)| {
                let display_name = display_name.try_into()?;
                let filter = Equal(Cow::Owned(display_name)).into();
                Ok(filter)
            })
            .transpose()?;
        let ne = ne
            .map(|NotEqual(display_name)| {
                let display_name = display_name.try_into()?;
                let filter = NotEqual(Cow::Owned(display_name)).into();
                Ok(filter)
            })
            .transpose()?;
        let r#in = r#in
            .map(|In(display_names)| {
                let display_names = display_names
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = In(display_names.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let nin = nin
            .map(|NotIn(display_names)| {
                let display_names = display_names
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = NotIn(display_names.into()).into();
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
