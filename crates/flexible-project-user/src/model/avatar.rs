use derive_more::Display;
use fp_user_domain::model::{Avatar as DomainAvatar, AvatarError};
use serde::{Deserialize, Serialize};

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
