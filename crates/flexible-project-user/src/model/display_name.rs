use derive_more::Display;
use fp_user_domain::model::{DisplayName as DomainDisplayName, DisplayNameError};
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
