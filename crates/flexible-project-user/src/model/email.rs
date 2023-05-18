use derive_more::Display;
use fp_user_domain::model::{Email as DomainEmail, EmailError};
use serde::{Deserialize, Serialize};

/// Serializable [email](DomainEmail) of the user.
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl From<DomainEmail> for Email {
    fn from(email: DomainEmail) -> Self {
        let email = email.into_inner();
        Self(email)
    }
}

impl TryFrom<Email> for DomainEmail {
    type Error = EmailError;

    fn try_from(email: Email) -> Result<Self, Self::Error> {
        let Email(email) = email;
        DomainEmail::new(email)
    }
}
