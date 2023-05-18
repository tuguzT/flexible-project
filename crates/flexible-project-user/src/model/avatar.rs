use derive_more::Display;
use fp_user_domain::model::{Avatar as DomainAvatar, AvatarError};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::filter::{Equal, In, NotEqual, NotIn, Regex};

/// Serializable [avatar](DomainAvatar) of the user.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Avatar(String);

impl From<DomainAvatar> for Avatar {
    fn from(avatar: DomainAvatar) -> Self {
        let avatar = avatar.into_inner();
        Self(avatar)
    }
}

impl TryFrom<Avatar> for DomainAvatar {
    type Error = AvatarError;

    fn try_from(avatar: Avatar) -> Result<Self, Self::Error> {
        let Avatar(avatar) = avatar;
        DomainAvatar::new(avatar)
    }
}

/// Filters for user avatar URL of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct AvatarFilters {
    /// Equality user avatar filter.
    pub eq: Option<Equal<Avatar>>,
    /// Inequality user avatar filter.
    pub ne: Option<NotEqual<Avatar>>,
    /// In user avatar filter.
    pub r#in: Option<In<Vec<Avatar>>>,
    /// Not in user avatar filter.
    pub nin: Option<NotIn<Vec<Avatar>>>,
    /// Regex user avatar filter.
    pub regex: Option<Regex<String>>,
}

/// Filters for user avatar URL of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct OptionAvatarFilters {
    /// Equality user avatar filter.
    pub eq: Option<Equal<Option<Avatar>>>,
    /// Inequality user avatar filter.
    pub ne: Option<NotEqual<Option<Avatar>>>,
    /// In user avatar filter.
    pub r#in: Option<In<Vec<Option<Avatar>>>>,
    /// Not in user avatar filter.
    pub nin: Option<NotIn<Vec<Option<Avatar>>>>,
    /// Regex user avatar filter.
    pub regex: Option<Regex<String>>,
}
