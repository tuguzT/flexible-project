use derive_more::Display;
use fp_user_domain::model::{DisplayName as DomainDisplayName, DisplayNameError};
use serde::{Deserialize, Serialize};

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
