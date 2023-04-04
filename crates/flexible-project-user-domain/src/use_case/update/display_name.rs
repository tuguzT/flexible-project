use derive_more::{Display, Error, From};

use crate::{
    model::{DisplayName, User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::find_one_by_id,
};

/// Error type of update user display name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateDisplayNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = r#"no user exists by identifier "{}""#, _0)]
    #[from(ignore)]
    NoUser(#[error(not(source))] UserId),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update display name interactor.
pub struct UpdateDisplayName<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> UpdateDisplayName<Database>
where
    Database: UserDatabase,
{
    /// Creates new update display name interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Updates display name of the user by its identifier with provided display name.
    pub async fn update_display_name(
        &self,
        id: UserId,
        display_name: DisplayName,
    ) -> Result<User, UpdateDisplayNameError<Database::Error>> {
        let Self { database } = self;

        let User { id, data } = {
            let user_by_id = find_one_by_id(database, &id).await?;
            user_by_id.ok_or_else(|| UpdateDisplayNameError::NoUser(id))?
        };
        let data = UserData {
            display_name,
            ..data
        };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
