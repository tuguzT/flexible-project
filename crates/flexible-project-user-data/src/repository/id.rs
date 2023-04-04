use std::convert::Infallible;

use fp_core::id::GenerateId;
use fp_user_domain::model::{User, UserId};

use crate::model::LocalUserId;

/// Implementation of user identifier generator.
#[derive(Debug, Default, Clone)]
pub struct LocalGenerateUserId;

impl GenerateId<User> for LocalGenerateUserId {
    type Error = Infallible;

    fn generate_id(&self) -> Result<UserId, Self::Error> {
        LocalUserId::new().try_into()
    }
}
