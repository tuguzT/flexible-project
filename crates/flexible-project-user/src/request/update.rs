use fp_user_domain::use_case::UpdateUserInput as DomainUpdateUserInput;
use serde::{Deserialize, Serialize};

use crate::model::{Avatar, DisplayName, Email, Name, TryFromUserDataError};

/// Serializable [input](DomainUpdateUserInput) of the update user interactor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserInput {
    /// Name of the user to update, if present.
    pub name: Option<Name>,
    /// Display name of the user to update, if present.
    pub display_name: Option<DisplayName>,
    /// Email of the user to update, if present.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub email: Option<Option<Email>>,
    /// Avatar of the user to update, if present.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub avatar: Option<Option<Avatar>>,
}

impl From<DomainUpdateUserInput> for UpdateUserInput {
    fn from(input: DomainUpdateUserInput) -> Self {
        let DomainUpdateUserInput {
            name,
            display_name,
            email,
            avatar,
        } = input;
        Self {
            name: name.map(Into::into),
            display_name: display_name.map(Into::into),
            email: email.map(|email| email.map(Into::into)),
            avatar: avatar.map(|avatar| avatar.map(Into::into)),
        }
    }
}

impl TryFrom<UpdateUserInput> for DomainUpdateUserInput {
    type Error = TryFromUserDataError;

    fn try_from(input: UpdateUserInput) -> Result<Self, Self::Error> {
        let UpdateUserInput {
            name,
            display_name,
            email,
            avatar,
        } = input;
        let input = Self {
            name: name.map(TryInto::try_into).transpose()?,
            display_name: display_name.map(TryInto::try_into).transpose()?,
            email: email
                .map(|email| email.map(TryInto::try_into).transpose())
                .transpose()?,
            avatar: avatar
                .map(|avatar| avatar.map(TryInto::try_into).transpose())
                .transpose()?,
        };
        Ok(input)
    }
}
