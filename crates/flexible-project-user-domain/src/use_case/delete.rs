use derive_more::{Display, Error, From};

use crate::model::{User, UserId};

use super::repository::{find_one_by_id, Repository};

/// Error type of delete user use case.
#[derive(Debug, Display, From, Error)]
pub enum DeleteUserError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Deletes user by provided identifier.
pub async fn delete_user<R>(repository: R, id: UserId) -> Result<User, DeleteUserError<R::Error>>
where
    R: Repository,
{
    let id_exists = {
        let user_by_id = find_one_by_id(&repository, &id).await?;
        user_by_id.is_some()
    };
    if !id_exists {
        return Err(DeleteUserError::NoUser);
    }

    let user = repository.delete(id).await?;
    Ok(user)
}
