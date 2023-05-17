use derive_more::{Display, Error, From};

use crate::{
    model::{Avatar, User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::find_one_by_id,
};

/// Error type of update user avatar use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateAvatarError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = r#"no user exists by identifier "{}""#, _0)]
    #[from(ignore)]
    NoUser(#[error(not(source))] UserId),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update avatar interactor.
pub struct UpdateAvatar<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> UpdateAvatar<Database>
where
    Database: UserDatabase,
{
    /// Creates new update avatar interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Updates avatar of the user by its identifier with provided avatar.
    pub async fn update_avatar(
        &self,
        current_id: UserId,
        avatar: Option<Avatar>,
    ) -> Result<User, UpdateAvatarError<Database::Error>> {
        let Self { database } = self;

        let User { id, data } = {
            let user_by_id = find_one_by_id(database, &current_id).await?;
            user_by_id.ok_or_else(|| UpdateAvatarError::NoUser(current_id))?
        };
        let data = UserData { avatar, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
