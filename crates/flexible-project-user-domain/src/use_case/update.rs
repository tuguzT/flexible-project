use derive_more::{Display, Error, From};

use crate::model::{DisplayName, Email, Name, Role, User, UserData, UserId};

use super::repository::{find_one_by_email, find_one_by_id, find_one_by_name, Repository};

/// Error type of update user name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    AlreadyTaken,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates name of the user by its identifier with provided name.
pub async fn update_name<R>(
    repository: R,
    id: UserId,
    name: Name,
) -> Result<User, UpdateNameError<R::Error>>
where
    R: Repository,
{
    let is_name_unique = {
        let user_by_name = find_one_by_name(&repository, &name).await?;
        user_by_name.is_none()
    };
    if !is_name_unique {
        return Err(UpdateNameError::AlreadyTaken);
    }

    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateNameError::NoUser)?
    };
    let data = UserData { name, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of update user display name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateDisplayNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates display name of the user by its identifier with provided display name.
pub async fn update_display_name<R>(
    repository: R,
    id: UserId,
    display_name: DisplayName,
) -> Result<User, UpdateDisplayNameError<R::Error>>
where
    R: Repository,
{
    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateDisplayNameError::NoUser)?
    };
    let data = UserData {
        display_name,
        ..data
    };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of update user role use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateRoleError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates role of the user by its identifier with provided role.
pub async fn update_role<R>(
    repository: R,
    id: UserId,
    role: Role,
) -> Result<User, UpdateRoleError<R::Error>>
where
    R: Repository,
{
    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateRoleError::NoUser)?
    };
    let data = UserData { role, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}

/// Error type of update user email use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateEmailError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// User with provided email already exists.
    #[display(fmt = "user email is already taken")]
    AlreadyTaken,
    /// Repository error.
    #[display(fmt = "repository error: {}", _0)]
    Repository(Error),
}

/// Updates email of the user by its identifier with provided email.
pub async fn update_email<R>(
    repository: R,
    id: UserId,
    email: Option<Email>,
) -> Result<User, UpdateEmailError<R::Error>>
where
    R: Repository,
{
    if let Some(ref email) = email {
        let is_email_unique = {
            let user_by_email = find_one_by_email(&repository, email).await?;
            user_by_email.is_none()
        };
        if !is_email_unique {
            return Err(UpdateEmailError::AlreadyTaken);
        }
    }

    let User { id, data } = {
        let user_by_id = find_one_by_id(&repository, id).await?;
        user_by_id.ok_or(UpdateEmailError::NoUser)?
    };
    let data = UserData { email, ..data };
    let user = repository.update(id, data).await?;
    Ok(user)
}
