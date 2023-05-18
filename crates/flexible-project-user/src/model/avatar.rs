use std::borrow::Cow;

use derive_more::Display;
use fp_user_domain::model::{
    Avatar as DomainAvatar, AvatarError, AvatarFilters as DomainAvatarFilters,
    OptionAvatarFilters as DomainOptionAvatarFilters,
};
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

impl From<DomainAvatarFilters<'_>> for AvatarFilters {
    fn from(filters: DomainAvatarFilters<'_>) -> Self {
        let DomainAvatarFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        Self {
            eq: eq.map(|avatar| Equal(avatar.0.into_owned().into())),
            ne: ne.map(|avatar| NotEqual(avatar.0.into_owned().into())),
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

impl TryFrom<AvatarFilters> for DomainAvatarFilters<'_> {
    type Error = AvatarError;

    fn try_from(filters: AvatarFilters) -> Result<Self, Self::Error> {
        let AvatarFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        let eq = eq
            .map(|Equal(avatar)| {
                let avatar = avatar.try_into()?;
                let filter = Equal(Cow::Owned(avatar)).into();
                Ok(filter)
            })
            .transpose()?;
        let ne = ne
            .map(|NotEqual(avatar)| {
                let avatar = avatar.try_into()?;
                let filter = NotEqual(Cow::Owned(avatar)).into();
                Ok(filter)
            })
            .transpose()?;
        let r#in = r#in
            .map(|In(avatars)| {
                let avatars = avatars
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = In(avatars.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let nin = nin
            .map(|NotIn(avatars)| {
                let avatars = avatars
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = NotIn(avatars.into()).into();
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

impl From<DomainOptionAvatarFilters<'_>> for OptionAvatarFilters {
    fn from(filters: DomainOptionAvatarFilters<'_>) -> Self {
        let DomainOptionAvatarFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        Self {
            eq: eq.map(|avatar| Equal(avatar.0.into_owned().map(Into::into))),
            ne: ne.map(|avatar| NotEqual(avatar.0.into_owned().map(Into::into))),
            r#in: r#in.map(|r#in| {
                let cow_slice = r#in.0;
                let iter = cow_slice.0.iter();
                In(iter.cloned().map(|avatar| avatar.map(Into::into)).collect())
            }),
            nin: nin.map(|r#in| {
                let cow_slice = r#in.0;
                let iter = cow_slice.0.iter();
                NotIn(iter.cloned().map(|avatar| avatar.map(Into::into)).collect())
            }),
            regex: regex.map(|regex| Regex(regex.0.into_owned())),
        }
    }
}

impl TryFrom<OptionAvatarFilters> for DomainOptionAvatarFilters<'_> {
    type Error = AvatarError;

    fn try_from(filters: OptionAvatarFilters) -> Result<Self, Self::Error> {
        let OptionAvatarFilters {
            eq,
            ne,
            r#in,
            nin,
            regex,
        } = filters;
        let eq = eq
            .map(|Equal(avatar)| {
                let avatar = avatar.map(TryInto::try_into).transpose()?;
                let filter = Equal(Cow::Owned(avatar)).into();
                Ok(filter)
            })
            .transpose()?;
        let ne = ne
            .map(|NotEqual(avatar)| {
                let avatar = avatar.map(TryInto::try_into).transpose()?;
                let filter = NotEqual(Cow::Owned(avatar)).into();
                Ok(filter)
            })
            .transpose()?;
        let r#in = r#in
            .map(|In(avatars)| {
                let avatars = avatars
                    .into_iter()
                    .map(|avatar| avatar.map(TryInto::try_into).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = In(avatars.into()).into();
                Ok(filter)
            })
            .transpose()?;
        let nin = nin
            .map(|NotIn(avatars)| {
                let avatars = avatars
                    .into_iter()
                    .map(|avatar| avatar.map(TryInto::try_into).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                let filter = NotIn(avatars.into()).into();
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
