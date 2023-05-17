use std::hash::{Hash, Hasher};

use derive_more::{Display, Error, From};
use fp_user_domain::model::{
    Avatar, AvatarError, DisplayName, DisplayNameError, Email, EmailError, Name, NameError, User,
    UserData,
};
use serde::{Deserialize, Serialize};

use super::{
    id::{LocalUserId, LocalUserIdError},
    role::LocalRole,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalUser {
    #[serde(rename = "_id")]
    pub id: LocalUserId,
    pub data: LocalUserData,
}

impl PartialEq for LocalUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for LocalUser {}

impl Hash for LocalUser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TryFrom<User> for LocalUser {
    type Error = LocalUserIdError;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        let User { id, data } = value;
        let id = id.try_into()?;
        let data = data.into();
        Ok(Self { id, data })
    }
}

impl TryFrom<LocalUser> for User {
    type Error = LocalUserDataError;

    fn try_from(value: LocalUser) -> Result<Self, Self::Error> {
        let LocalUser { id, data } = value;
        let user = User {
            id: id.into(),
            data: data.try_into()?,
        };
        Ok(user)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalUserData {
    pub name: String,
    pub display_name: String,
    pub role: LocalRole,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

impl From<UserData> for LocalUserData {
    fn from(value: UserData) -> Self {
        let UserData {
            name,
            display_name,
            role,
            email,
            avatar,
        } = value;
        Self {
            name: name.into_inner(),
            display_name: display_name.into_inner(),
            role: role.into(),
            email: email.map(Email::into_inner),
            avatar: avatar.map(Avatar::into_inner),
        }
    }
}

impl TryFrom<LocalUserData> for UserData {
    type Error = LocalUserDataError;

    fn try_from(value: LocalUserData) -> Result<Self, Self::Error> {
        let LocalUserData {
            name,
            display_name,
            role,
            email,
            avatar,
        } = value;
        let user_data = Self {
            name: Name::new(name)?,
            display_name: DisplayName::new(display_name)?,
            role: role.into(),
            email: email.map(Email::new).transpose()?,
            avatar: avatar.map(Avatar::new).transpose()?,
        };
        Ok(user_data)
    }
}

#[derive(Debug, Display, Clone, Copy, From, Error)]
pub enum LocalUserDataError {
    Name(NameError),
    DisplayName(DisplayNameError),
    Email(EmailError),
    Avatar(AvatarError),
}
