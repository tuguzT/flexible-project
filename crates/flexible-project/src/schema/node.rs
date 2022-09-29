//! Definitions of node queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Error, Object, ID};
use fp_data::repository::ops::ReadById;

use crate::data::UserRepositoryData;
use crate::model::{Node, User};

/// Node query object of the Flexible Project system.
#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    /// Returns data of the node by provided ID.
    async fn node(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The ID of the object.")] id: ID,
    ) -> Result<Option<Node>, Error> {
        // TODO: find from more than one repository (add special use case for node searching by ID)
        let id = id.parse()?;
        let repository = ctx
            .data::<UserRepositoryData>()
            .expect("user repository should always exist");
        let node = repository
            .read_by_id(id)
            .await?
            .map(User::from)
            .map(Node::from);
        Ok(node)
    }
}
