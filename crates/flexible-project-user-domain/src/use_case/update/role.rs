use derive_more::{Display, Error, From};

use crate::{
    model::{Role, User, UserData, UserId},
    repository::UserDatabase,
    use_case::ext::UserDatabaseExt,
};

/// Error type of update user role use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateRoleError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update role interactor.
pub struct UpdateRole<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> UpdateRole<Db>
where
    Db: UserDatabase,
{
    /// Creates new update role interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Updates role of the user by its identifier with provided role.
    pub async fn update_role(
        &self,
        id: UserId,
        role: Role,
    ) -> Result<User, UpdateRoleError<Db::Error>> {
        let Self { database } = self;

        let User { id, data } = {
            let user_by_id = database.find_one_by_id(id).await?;
            user_by_id.ok_or(UpdateRoleError::NoUser)?
        };
        let data = UserData { role, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
