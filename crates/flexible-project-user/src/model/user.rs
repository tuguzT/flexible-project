use std::hash::{Hash, Hasher};

use derive_more::{Display, Error, From};
use fp_core::id::{ErasedId as CoreErasedId, ErasedIdFilters as CoreErasedIdFilters};
use fp_user_domain::model::{
    AvatarError, DisplayNameError, EmailError, NameError, User as DomainUser,
    UserData as DomainUserData, UserDataFilters as DomainUserDataFilters,
    UserFilters as DomainUserFilters,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{
    Avatar, DisplayName, DisplayNameFilters, Email, ErasedId, ErasedIdFilters, Name, NameFilters,
    OptionAvatarFilters, OptionEmailFilters, Role, RoleFilters,
};

/// Serializable [user](DomainUser) of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier of the user.
    pub id: ErasedId,
    /// Data of the user.
    #[serde(flatten)]
    pub data: UserData,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl From<DomainUser> for User {
    fn from(user: DomainUser) -> Self {
        let DomainUser { id, data } = user;
        Self {
            id: id.erase().into(),
            data: data.into(),
        }
    }
}

impl TryFrom<User> for DomainUser {
    type Error = TryFromUserDataError;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        let User { id, data } = user;
        let user = Self {
            id: CoreErasedId::from(id).with_owner(),
            data: data.try_into()?,
        };
        Ok(user)
    }
}

/// Filters for user of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserFilters {
    /// User identifier filters.
    pub id: Option<ErasedIdFilters>,
    /// User data filters.
    pub data: Option<UserDataFilters>,
}

impl From<DomainUserFilters<'_>> for UserFilters {
    fn from(filters: DomainUserFilters<'_>) -> Self {
        let DomainUserFilters { id, data } = filters;
        Self {
            id: id.map(|id| id.erase().into()),
            data: data.map(Into::into),
        }
    }
}

impl TryFrom<UserFilters> for DomainUserFilters<'_> {
    type Error = TryFromUserDataError;

    fn try_from(filters: UserFilters) -> Result<Self, Self::Error> {
        let UserFilters { id, data } = filters;
        let filters = Self {
            id: id.map(|id| CoreErasedIdFilters::from(id).with_owner()),
            data: data.map(TryInto::try_into).transpose()?,
        };
        Ok(filters)
    }
}

/// Serializable [user data](DomainUserData) of the system.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserData {
    /// Unique name of the user.
    pub name: Name,
    /// Display name of the user.
    pub display_name: DisplayName,
    /// Role of the user.
    pub role: Role,
    /// Unique email of the user, if present.
    pub email: Option<Email>,
    /// Avatar URL of the user, if present.
    pub avatar: Option<Avatar>,
}

impl From<DomainUserData> for UserData {
    fn from(data: DomainUserData) -> Self {
        let DomainUserData {
            name,
            display_name,
            role,
            email,
            avatar,
        } = data;
        Self {
            name: name.into(),
            display_name: display_name.into(),
            role: role.into(),
            email: email.map(Into::into),
            avatar: avatar.map(Into::into),
        }
    }
}

impl TryFrom<UserData> for DomainUserData {
    type Error = TryFromUserDataError;

    fn try_from(data: UserData) -> Result<Self, Self::Error> {
        let UserData {
            name,
            display_name,
            role,
            email,
            avatar,
        } = data;
        let data = Self {
            name: name.try_into()?,
            display_name: display_name.try_into()?,
            role: role.into(),
            email: email.map(TryInto::try_into).transpose()?,
            avatar: avatar.map(TryInto::try_into).transpose()?,
        };
        Ok(data)
    }
}

/// Filters for user data of the backend.
#[derive(Debug, Clone, Default, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct UserDataFilters {
    /// User name filters.
    pub name: Option<NameFilters>,
    /// User display name filters.
    pub display_name: Option<DisplayNameFilters>,
    /// User role filters.
    pub role: Option<RoleFilters>,
    /// User email filters.
    pub email: Option<OptionEmailFilters>,
    /// User avatar filters.
    pub avatar: Option<OptionAvatarFilters>,
}

impl From<DomainUserDataFilters<'_>> for UserDataFilters {
    fn from(filters: DomainUserDataFilters<'_>) -> Self {
        let DomainUserDataFilters {
            name,
            display_name,
            role,
            email,
            avatar,
        } = filters;
        Self {
            name: name.map(Into::into),
            display_name: display_name.map(Into::into),
            role: role.map(Into::into),
            email: email.map(Into::into),
            avatar: avatar.map(Into::into),
        }
    }
}

impl TryFrom<UserDataFilters> for DomainUserDataFilters<'_> {
    type Error = TryFromUserDataError;

    fn try_from(filters: UserDataFilters) -> Result<Self, Self::Error> {
        let UserDataFilters {
            name,
            display_name,
            role,
            email,
            avatar,
        } = filters;
        let filters = Self {
            name: name.map(TryInto::try_into).transpose()?,
            display_name: display_name.map(TryInto::try_into).transpose()?,
            role: role.map(Into::into),
            email: email.map(TryInto::try_into).transpose()?,
            avatar: avatar.map(TryInto::try_into).transpose()?,
        };
        Ok(filters)
    }
}

/// Type of error which is returned when serializable user data
/// cannot be converted into domain user data.
#[derive(Debug, Display, Clone, Copy, Error, From)]
pub enum TryFromUserDataError {
    /// Name does not meet domain requirements.
    Name(NameError),
    /// Display name does not meet domain display name requirements.
    DisplayName(DisplayNameError),
    /// Email does not meet domain requirements.
    Email(EmailError),
    /// Avatar does not meet domain requirements.
    Avatar(AvatarError),
}
