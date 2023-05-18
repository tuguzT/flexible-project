use derive_more::Display;
use fp_user_domain::model::{Name as DomainName, NameError};
use serde::{Deserialize, Serialize};

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
