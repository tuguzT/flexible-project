use derive_more::{Display, Error};

use crate::{
    model::{User, UserData},
    repository::{IdGenerator, UserDatabase},
};

use super::ext::UserDatabaseExt;

/// Error type of create user use case.
#[derive(Debug, Display, Error)]
pub enum CreateUserError<DbError, IdGenError> {
    /// Identifier generation error.
    #[display(fmt = "identifier generation error: {}", _0)]
    IdGeneration(IdGenError),
    /// User with provided name already exists.
    #[display(fmt = "user name is already taken")]
    NameAlreadyTaken,
    /// User with provided email already exists.
    #[display(fmt = "user email is already taken")]
    EmailAlreadyTaken,
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(DbError),
}

/// Create user interactor.
pub struct CreateUser<Db, IdGen>
where
    Db: UserDatabase,
    IdGen: IdGenerator,
{
    database: Db,
    id_generator: IdGen,
}

impl<Db, IdGen> CreateUser<Db, IdGen>
where
    Db: UserDatabase,
    IdGen: IdGenerator,
{
    /// Creates new create user interactor.
    pub fn new(database: Db, id_generator: IdGen) -> Self {
        Self {
            database,
            id_generator,
        }
    }

    /// Creates new user from provided identifier and user data.
    pub async fn create_user(
        &self,
        data: UserData,
    ) -> Result<User, CreateUserError<Db::Error, IdGen::Error>> {
        let Self {
            database,
            id_generator,
        } = self;

        let id = id_generator
            .generate_id()
            .map_err(CreateUserError::IdGeneration)?;

        let UserData { ref name, .. } = data;
        let is_name_unique = {
            let user_by_name = database
                .find_one_by_name(name)
                .await
                .map_err(CreateUserError::Database)?;
            user_by_name.is_none()
        };
        if !is_name_unique {
            return Err(CreateUserError::NameAlreadyTaken);
        }

        let UserData { ref email, .. } = data;
        if email.is_some() {
            let is_email_unique = {
                let user_by_email = database
                    .find_one_by_email(email)
                    .await
                    .map_err(CreateUserError::Database)?;
                user_by_email.is_none()
            };
            if !is_email_unique {
                return Err(CreateUserError::EmailAlreadyTaken);
            }
        }

        let user = database
            .create(id, data)
            .await
            .map_err(CreateUserError::Database)?;
        Ok(user)
    }
}
