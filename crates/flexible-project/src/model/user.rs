use async_graphql::{Context, Enum, Object, SimpleObject};
use fp_core::model::{Identifiable, User};
use fp_data::repository::ops::ReadAll;

use crate::data::UserRepositoryData;
use crate::model::id::IdData;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    pub async fn users(&self, ctx: &Context<'_>) -> Vec<UserData> {
        let repository = ctx.data_unchecked::<UserRepositoryData>();
        let repository = repository.read().await;
        let users = repository.read_all().await;
        users.into_iter().map(Into::into).collect()
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
pub struct UserData {
    pub id: IdData<Self>,
    pub name: String,
    pub email: Option<String>,
    pub role: UserRole,
}

impl Identifiable for UserData {
    type Id = IdData<Self>;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl User for UserData {
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

impl From<fp_data::model::UserData> for UserData {
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
