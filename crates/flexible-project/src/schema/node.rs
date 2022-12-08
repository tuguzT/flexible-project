//! Definitions of node queries, mutations and subscriptions of the Flexible Project system.

use async_graphql::{Context, Object, Result, ID};

use crate::model::node::Node;

use super::di::interactor::node::FindNode;
use super::ext::ContextExt;

/// Node query object of the Flexible Project system.
#[derive(Debug, Default)]
pub struct NodeQuery(());

#[Object]
impl NodeQuery {
    /// Returns data of the node by provided ID.
    async fn node(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The ID of the object.")] id: ID,
    ) -> Result<Option<Node>> {
        let interactor = ctx.resolve_ref::<dyn FindNode>();
        let id = id.to_string().into();
        let node = interactor.find(id).await?.map(Node::from);
        Ok(node)
    }
}
