use derive_more::{Display, Error, From};

use crate::{
    model::{Name, User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::{find_one_by_id, find_one_by_name},
};

/// Error type of update user name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    AlreadyTaken,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update name interactor.
pub struct UpdateName<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> UpdateName<Db>
where
    Db: UserDatabase,
{
    /// Creates new update name interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Updates name of the user by its identifier with provided name.
    pub async fn update_name(
        &self,
        id: UserId,
        name: Name,
    ) -> Result<User, UpdateNameError<Db::Error>> {
        let Self { database } = self;

        let is_name_unique = {
            let user_by_name = find_one_by_name(database, &name).await?;
            user_by_name.is_none()
        };
        if !is_name_unique {
            return Err(UpdateNameError::AlreadyTaken);
        }

        let User { id, data } = {
            let user_by_id = find_one_by_id(database, id).await?;
            user_by_id.ok_or(UpdateNameError::NoUser)?
        };
        let data = UserData { name, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
