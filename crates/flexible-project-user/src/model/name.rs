use derive_more::Display;
use fp_user_domain::model::{Name as DomainName, NameError};
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
