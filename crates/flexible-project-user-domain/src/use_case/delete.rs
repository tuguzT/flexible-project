use derive_more::{Display, Error, From};

use crate::{
    model::{User, UserId},
    repository::UserDatabase,
};

use super::find_one::find_one_by_id;

/// Error type of delete user use case.
#[derive(Debug, Display, From, Error)]
pub enum DeleteUserError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Delete user interactor.
pub struct DeleteUser<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> DeleteUser<Db>
where
    Db: UserDatabase,
{
    /// Creates new delete user interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Deletes user by provided identifier.
    pub async fn delete_user(&self, id: UserId) -> Result<User, DeleteUserError<Db::Error>> {
        let Self { database } = self;

        let id_exists = {
            let user_by_id = find_one_by_id(database, &id).await?;
            user_by_id.is_some()
        };
        if !id_exists {
            return Err(DeleteUserError::NoUser);
        }

        let user = database.delete(id).await?;
        Ok(user)
    }
}
