use derive_more::{Display, Error, From};

use crate::{
    model::{Avatar, DisplayName, Email, Name, User, UserData, UserId},
    repository::UserDatabase,
    use_case::find_one::{find_one_by_email, find_one_by_id, find_one_by_name},
};

/// Error type of update user use case.
#[derive(Debug, Display, From, Error)]
pub enum UpdateUserError<Error> {
    /// No user was found by provided identifier.
    #[display(fmt = r#"no user exists by identifier "{}""#, _0)]
    #[from(ignore)]
    NoUser(#[error(not(source))] UserId),
    /// User with provided name already exists.
    #[display(fmt = r#"user name "{}" is already taken"#, _0)]
    #[from(ignore)]
    NameAlreadyTaken(#[error(not(source))] Name),
    /// User with provided email already exists.
    #[display(fmt = r#"user email "{}" is already taken"#, _0)]
    #[from(ignore)]
    EmailAlreadyTaken(#[error(not(source))] Email),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(Error),
}

/// Update user interactor.
pub struct UpdateUser<Database>
where
    Database: UserDatabase,
{
    database: Database,
}

impl<Database> UpdateUser<Database>
where
    Database: UserDatabase,
{
    /// Creates new update user interactor.
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Updates user by its identifier with provided name, display name and email.
    pub async fn update_user(
        &self,
        current_id: UserId,
        name: Option<Name>,
        display_name: Option<DisplayName>,
        email: Option<Option<Email>>,
        avatar: Option<Option<Avatar>>,
    ) -> Result<User, UpdateUserError<Database::Error>> {
        let Self { database } = self;

        let User { id, mut data } = {
            let user_by_id = find_one_by_id(database, &current_id).await?;
            user_by_id.ok_or_else(|| UpdateUserError::NoUser(current_id))?
        };
        if let Some(name) = name {
            let user_by_name = find_one_by_name(database, &name).await?;
            if let Some(user_by_name) = user_by_name {
                let User { data, .. } = user_by_name;
                let UserData { name, .. } = data;
                return Err(UpdateUserError::NameAlreadyTaken(name));
            }
            data.name = name;
        }
        if let Some(display_name) = display_name {
            data.display_name = display_name;
        }
        if let Some(email) = email {
            if email.is_some() {
                let user_by_email = find_one_by_email(database, &email).await?;
                if let Some(user_by_email) = user_by_email {
                    let User { data, .. } = user_by_email;
                    let UserData { email, .. } = data;
                    let email = email.expect("user was found by email which is `Some`");
                    return Err(UpdateUserError::EmailAlreadyTaken(email));
                }
            }
            data.email = email;
        }
        if let Some(avatar) = avatar {
            data.avatar = avatar;
        }

        let user = database.update(id, data).await?;
        Ok(user)
    }
}
