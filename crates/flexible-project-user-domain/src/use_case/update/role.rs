use derive_more::{Display, Error, From};

use crate::{
    model::{Role, User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::find_one_by_id,
};

/// Error type of update user role use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateRoleError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = r#"no user exists by identifier "{}""#, _0)]
    #[from(ignore)]
    NoUser(#[error(not(source))] UserId),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update role interactor.
pub struct UpdateRole<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> UpdateRole<Database>
where
    Database: UserDatabase,
{
    /// Creates new update role interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Updates role of the user by its identifier with provided role.
    pub async fn update_role(
        &self,
        id: UserId,
        role: Role,
    ) -> Result<User, UpdateRoleError<Database::Error>> {
        let Self { database } = self;

        let User { id, data } = {
            let user_by_id = find_one_by_id(database, &id).await?;
            user_by_id.ok_or_else(|| UpdateRoleError::NoUser(id))?
        };
        let data = UserData { role, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
