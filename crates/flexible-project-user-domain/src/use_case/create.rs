use derive_more::{Display, Error, From};

use crate::model::{User, UserData, UserId};

use super::repository::{find_one_by_id, find_one_by_name, Repository};

/// Error type of create user use case.
#[derive(Debug, Display, From, Error)]
pub enum CreateUserError<Error> {
    /// User with provided identifier already exists.
    #[display(fmt = "identifier is already taken")]
    IdAlreadyTaken,
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    NameAlreadyTaken,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Creates new user from provided identifier and user data.
pub async fn create_user<R>(
    repository: R,
    id: UserId,
    data: UserData,
) -> Result<User, CreateUserError<R::Error>>
where
    R: Repository,
{
    let id_exists = {
        let user_by_id = find_one_by_id(&repository, &id).await?;
        user_by_id.is_some()
    };
    if id_exists {
        return Err(CreateUserError::IdAlreadyTaken);
    }

    let UserData { ref name, .. } = data;
    let is_name_unique = {
        let user_by_name = find_one_by_name(&repository, name).await?;
        user_by_name.is_none()
    };
    if !is_name_unique {
        return Err(CreateUserError::NameAlreadyTaken);
    }

    let user = repository.create(id, data).await?;
    Ok(user)
}
