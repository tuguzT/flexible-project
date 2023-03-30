use derive_more::{Display, Error, From};

use crate::{
    model::{User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::{find_one_by_email, find_one_by_id, find_one_by_name},
};

/// Error type of update user use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateUserError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    NameAlreadyTaken,
    /// User with provided email already exists.
    #[display(fmt = "user email is already taken")]
    EmailAlreadyTaken,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update user interactor.
pub struct UpdateUser<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> UpdateUser<Db>
where
    Db: UserDatabase,
{
    /// Creates new update user interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Updates user by its identifier with provided data.
    pub async fn update_user(
        &self,
        id: UserId,
        data: UserData,
    ) -> Result<User, UpdateUserError<Db::Error>> {
        let Self { database } = self;

        let UserData { ref name, .. } = data;
        let is_name_unique = {
            let user_by_name = find_one_by_name(database, name).await?;
            user_by_name.is_none()
        };
        if !is_name_unique {
            return Err(UpdateUserError::NameAlreadyTaken);
        }

        let UserData { ref email, .. } = data;
        if email.is_some() {
            let is_email_unique = {
                let user_by_email = find_one_by_email(database, email).await?;
                user_by_email.is_none()
            };
            if !is_email_unique {
                return Err(UpdateUserError::EmailAlreadyTaken);
            }
        }

        let User { id, .. } = {
            let user_by_id = find_one_by_id(database, id).await?;
            user_by_id.ok_or(UpdateUserError::NoUser)?
        };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
