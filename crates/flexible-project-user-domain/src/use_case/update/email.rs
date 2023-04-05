use derive_more::{Display, Error, From};

use crate::{
    model::{Email, User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::{find_one_by_email, find_one_by_id},
};

/// Error type of update user email use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateEmailError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = r#"no user exists by identifier "{}""#, _0)]
    #[from(ignore)]
    NoUser(#[error(not(source))] UserId),
    /// User with provided email already exists.
    #[display(fmt = r#"user email "{}" is already taken"#, _0)]
    #[from(ignore)]
    AlreadyTaken(#[error(not(source))] Email),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update email interactor.
pub struct UpdateEmail<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> UpdateEmail<Database>
where
    Database: UserDatabase,
{
    /// Creates new update email interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Updates email of the user by its identifier with provided email.
    pub async fn update_email(
        &self,
        current_id: UserId,
        email: Option<Email>,
    ) -> Result<User, UpdateEmailError<Database::Error>> {
        let Self { database } = self;

        if email.is_some() {
            let user_by_email = find_one_by_email(database, &email).await?;
            if let Some(user_by_email) = user_by_email {
                let User { data, .. } = user_by_email;
                let UserData { email, .. } = data;
                let email = email.expect("user was found by email which is `Some`");
                return Err(UpdateEmailError::AlreadyTaken(email));
            }
        }

        let User { id, data } = {
            let user_by_id = find_one_by_id(database, &current_id).await?;
            user_by_id.ok_or_else(|| UpdateEmailError::NoUser(current_id))?
        };
        let data = UserData { email, ..data };
        let user = database.update(id, data).await?;
        Ok(user)
    }
}
