use derive_more::{Display, Error, From};

use crate::{
    model::{DisplayName, User, UserData, UserId},
    repository::UserDatabase,
    use_case::ext::UserDatabaseExt,
};

/// Error type of update user display name use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateDisplayNameError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    #[from(ignore)]
    NoUser,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update display name interactor.
pub struct UpdateDisplayName<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> UpdateDisplayName<Db>
where
    Db: UserDatabase,
{
    /// Creates new update display name interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Updates display name of the user by its identifier with provided display name.
    pub async fn update_display_name(
        &self,
        id: UserId,
        display_name: DisplayName,
    ) -> Result<User, UpdateDisplayNameError<Db::Error>> {
        let Self { database } = self;

        let User { id, data } = {
            let user_by_id = database.find_one_by_id(id).await?;
            user_by_id.ok_or(UpdateDisplayNameError::NoUser)?
        };
        let data = UserData {
            display_name,
            ..data
        };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
