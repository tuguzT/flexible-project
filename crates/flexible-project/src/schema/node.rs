//! Definitions of node queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Error, Object, ID};
use fp_core::use_case::FindNode as _;
use fp_data::interactor::FindNode;

use crate::model::Node;

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
        let interactor = ctx
            .data::<FindNode>()
            .expect("find node interactor should always exist");
        let id = id.to_string().into();
        let node = interactor.find(id).await?.map(Node::from);
        Ok(node)
    }
}
