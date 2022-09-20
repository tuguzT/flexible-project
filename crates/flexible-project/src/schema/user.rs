//! Definitions of user queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Object};
use fp_data::repository::ops::{ReadAll, ReadById};

use crate::data::UserRepositoryData;
use crate::model::{Id, User};

/// GraphQL user query object of the Flexible Project system.
#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    /// Data of all users of the Flexible Project system.
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let repository = repository.read().await;
        let users = repository.read_all().await;
        users.into_iter().map(User::from).collect()
    }

    /// Data of user by its identifier of the Flexible Project system.
    async fn user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Unique identifier of the user.")] id: Id<User>,
    ) -> Option<User> {
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let repository = repository.read().await;
        let id = String::from(id).into();
        let user = repository.read_by_id(id).await;
        user.map(User::from)
    }
}
