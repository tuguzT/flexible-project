use std::convert::Infallible;

use domain::{model::UserId, repository::IdGenerator};

use crate::model::LocalUserId;

/// Implementation of user identifier generator.
#[derive(Debug, Default, Clone)]
pub struct LocalIdGenerator;

impl IdGenerator for LocalIdGenerator {
    type Error = Infallible;

    fn generate_id(&self) -> Result<UserId, Self::Error> {
        LocalUserId::new().try_into()
    }
}
