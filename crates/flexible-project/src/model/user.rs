use async_graphql::{Context, Enum, Object, SimpleObject};
use fp_core::model::Identifiable;
use fp_data::repository::ops::{ReadAll, ReadById};

use crate::data::UserRepositoryData;
use crate::model::id::Id;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let repository = ctx.data_unchecked::<UserRepositoryData>();
        let repository = repository.read().await;
        let users = repository.read_all().await;
        users.into_iter().map(Into::into).collect()
    }

    async fn user(&self, ctx: &Context<'_>, id: Id<User>) -> Option<User> {
        let repository = ctx.data_unchecked::<UserRepositoryData>();
        let repository = repository.read().await;
        let id: String = id.into();
        let id = id.into();
        let user = repository.read_by_id(id).await;
        user.map(Into::into)
    }
}

#[derive(Enum, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[graphql(remote = "fp_core::model::UserRole")]
pub enum UserRole {
    #[default]
    User,
    Moderator,
    Administrator,
}

#[derive(SimpleObject)]
pub struct User {
    pub id: Id<Self>,
    pub name: String,
    pub email: Option<String>,
    pub role: UserRole,
}

impl Identifiable for User {
    type Id = Id<Self>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl fp_core::model::User for User {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    fn role(&self) -> fp_core::model::UserRole {
        self.role.into()
    }
}

impl From<fp_data::model::UserData> for User {
    fn from(user_data: fp_data::model::UserData) -> Self {
        let id: String = user_data.id.into();
        Self {
            id: id.into(),
            name: user_data.name,
            email: user_data.email,
            role: user_data.role.into(),
        }
    }
}

impl From<User> for fp_data::model::UserData {
    fn from(user_data: User) -> Self {
        let id: String = user_data.id.into();
        Self {
            id: id.into(),
            name: user_data.name,
            email: user_data.email,
            role: user_data.role.into(),
        }
    }
}
