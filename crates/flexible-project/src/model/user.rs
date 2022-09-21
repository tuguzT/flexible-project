use async_graphql::{ComplexObject, Enum, InputObject, SimpleObject, ID};
use fp_core::model::{Identifiable, User as CoreUser, UserRole as CoreUserRole};
use fp_data::model::{Id, User as DataUser};
use uuid::Uuid;

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
#[derive(SimpleObject)]
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
    async fn id(&self) -> ID {
        self.id.clone().into()
    }
}

impl Identifiable for User {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl CoreUser for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn role(&self) -> CoreUserRole {
        self.role.into()
    }
}

impl From<DataUser> for User {
    fn from(user: DataUser) -> Self {
        Self {
            id: Uuid::from(user.id).into(),
            name: user.name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

impl From<User> for DataUser {
    fn from(user: User) -> Self {
        Self {
            id: Uuid::from(user.id).into(),
            name: user.name,
            email: user.email,
            role: user.role.into(),
        }
    }
}

/// Necessary data for creating new user in the Flexible Project system.
#[derive(InputObject)]
pub struct NewUser {
    /// Unique name of the user.
    pub name: String,
    /// Unique email of the user, if exists.
    pub email: Option<String>,
}
