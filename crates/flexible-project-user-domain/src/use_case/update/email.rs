use derive_more::{Display, Error, From};

use crate::{
    model::{Email, User, UserData, UserId},
    repository::UserDatabase,
    use_case::ext::UserDatabaseExt,
};

/// Error type of update user email use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateEmailError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = "no user exists by identifier")]
    NoUser,
    /// User with provided email already exists.
    #[display(fmt = "user email is already taken")]
    AlreadyTaken,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update email interactor.
pub struct UpdateEmail<Db>
where
    Db: UserDatabase,
{
    database: Db,
}

impl<Db> UpdateEmail<Db>
where
    Db: UserDatabase,
{
    /// Creates new update email interactor.
    pub fn new(database: Db) -> Self {
        Self { database }
    }

    /// Updates email of the user by its identifier with provided email.
    pub async fn update_email(
        &self,
        id: UserId,
        email: Option<Email>,
    ) -> Result<User, UpdateEmailError<Db::Error>> {
        let Self { database } = self;

        if email.is_some() {
            let is_email_unique = {
                let user_by_email = database.find_one_by_email(&email).await?;
                user_by_email.is_none()
            };
            if !is_email_unique {
                return Err(UpdateEmailError::AlreadyTaken);
            }
        }

        let User { id, data } = {
            let user_by_id = database.find_one_by_id(id).await?;
            user_by_id.ok_or(UpdateEmailError::NoUser)?
        };
        let data = UserData { email, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
