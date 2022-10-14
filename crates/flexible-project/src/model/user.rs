use async_graphql::{ComplexObject, Enum, InputObject, SimpleObject, ID};
use fp_core::model::{
    Id, User as CoreUser, UserCredentials as CoreUserCredentials, UserRole as CoreUserRole,
};

/// Role of user in the Flexible Project system.
#[derive(Enum, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[graphql(remote = "CoreUserRole")]
pub enum UserRole {
    /// An ordinary user of the system.
    #[default]
    User,
    /// Role of a moderator of the system.
    Moderator,
    /// Role of an administrator of the system.
    Administrator,
}

/// User data in the Flexible Project system.
#[derive(SimpleObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[graphql(complex)]
pub struct User {
    /// Unique identifier of the user.
    #[graphql(skip)]
    pub id: Id<Self>,
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    pub role: UserRole,
}

#[ComplexObject]
impl User {
    /// Unique identifier of the user.
    pub async fn id(&self) -> ID {
        self.id.clone().into()
    }
}

impl From<CoreUser> for User {
    fn from(user: CoreUser) -> Self {
        Self {
            id: user.id.change_owner(),
            name: user.name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

impl From<User> for CoreUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.change_owner(),
            name: user.name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

/// User credentials in the Flexible Project system.
#[derive(InputObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserCredentials {
    /// Name of the user.
    pub name: String,
    /// Password of the user.
    #[graphql(secret)]
    pub password: String,
}

impl From<CoreUserCredentials> for UserCredentials {
    fn from(credentials: CoreUserCredentials) -> Self {
        Self {
            name: credentials.name,
            password: credentials.password,
        }
    }
}

impl From<UserCredentials> for CoreUserCredentials {
    fn from(credentials: UserCredentials) -> Self {
        Self {
            name: credentials.name,
            password: credentials.password,
        }
    }
}

/// User input data in the Flexible Project system.
#[derive(InputObject, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateUser {
    /// Unique identifier of the user.
    pub id: ID,
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
    /// Role of the user in the system.
    pub role: UserRole,
}

impl From<CoreUser> for UpdateUser {
    fn from(user: CoreUser) -> Self {
        Self {
            id: user.id.into(),
            name: user.name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

impl From<UpdateUser> for CoreUser {
    fn from(user: UpdateUser) -> Self {
        Self {
            id: user.id.to_string().into(),
            name: user.name,
            email: user.email,
            role: user.role.into(),
        }
    }
}
