use derive_more::{Display, Error};

use crate::{
    model::{DisplayName, Name, Role, User, UserData},
    repository::{GenerateUserId, UserDatabase},
};

use super::find_one::find_one_by_name;

/// Error type of create user use case.
#[derive(Debug, Display, Error)]
pub enum CreateUserError<DatabaseError, GenerateIdError> {
    /// User with provided name already exists.
    #[display(fmt = r#"user name "{}" is already taken"#, _0)]
    NameAlreadyTaken(#[error(not(source))] Name),
    /// Database error.
    #[display(fmt = "database error: {}", _0)]
    Database(DatabaseError),
    /// Identifier generation error.
    #[display(fmt = "identifier generation error: {}", _0)]
    GenerateId(GenerateIdError),
}

/// Create user interactor.
pub struct CreateUser<Database, GenerateId>
where
    Database: UserDatabase,
    GenerateId: GenerateUserId,
{
    database: Database,
    generate_id: GenerateId,
}

impl<Database, GenerateId> CreateUser<Database, GenerateId>
where
    Database: UserDatabase,
    GenerateId: GenerateUserId,
{
    /// Creates new create user interactor.
    pub fn new(database: Database, generate_id: GenerateId) -> Self {
        Self {
            database,
            generate_id,
        }
    }

    /// Creates new user from provided unique user name.
    pub async fn create_user(
        &self,
        name: Name,
    ) -> Result<User, CreateUserError<Database::Error, GenerateId::Error>> {
        let Self {
            database,
            generate_id,
        } = self;

        let id = generate_id
            .generate_id()
            .map_err(CreateUserError::GenerateId)?;

        let is_name_unique = {
            let user_by_name = find_one_by_name(database, &name)
                .await
                .map_err(CreateUserError::Database)?;
            user_by_name.is_none()
        };
        if !is_name_unique {
            return Err(CreateUserError::NameAlreadyTaken(name));
        }

        let display_name = DisplayName::new(name.as_str())
            .expect("provided name should match display name requirements");
        let data = UserData {
            display_name,
            name,
            role: Role::User,
            email: None,
        };
        let user = database
            .create(id, data)
            .await
            .map_err(CreateUserError::Database)?;
        Ok(user)
    }
}
