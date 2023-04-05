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
    #[display(fmt = r#"no user exists by identifier "{}""#, _0)]
    #[from(ignore)]
    NoUser(#[error(not(source))] UserId),
    /// User with provided name already exists.
    #[display(fmt = r#"user name "{}" is already taken"#, _0)]
    #[from(ignore)]
    AlreadyTaken(#[error(not(source))] Name),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update name interactor.
pub struct UpdateName<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> UpdateName<Database>
where
    Database: UserDatabase,
{
    /// Creates new update name interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Updates name of the user by its identifier with provided name.
    pub async fn update_name(
        &self,
        current_id: UserId,
        name: Name,
    ) -> Result<User, UpdateNameError<Database::Error>> {
        let Self { database } = self;

        let user_by_name = find_one_by_name(database, &name).await?;
        if let Some(user_by_name) = user_by_name {
            let User { data, .. } = user_by_name;
            let UserData { name, .. } = data;
            return Err(UpdateNameError::AlreadyTaken(name));
        }

        let User { id, data } = {
            let user_by_id = find_one_by_id(database, &current_id).await?;
            user_by_id.ok_or_else(|| UpdateNameError::NoUser(current_id))?
        };
        let data = UserData { name, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
