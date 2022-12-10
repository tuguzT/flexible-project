use std::sync::Arc;

use shaku::{Component, HasComponent, Interface, Module};

use super::user::FilterUsers;

mod core {
    pub use fp_core::use_case::node::FindNode;
}

mod data {
    pub use fp_data::interactor::node::FindNode;
}

pub trait FindNode: core::FindNode + Interface {}
impl<T> FindNode for T where T: core::FindNode + Interface {}

pub struct FindNodeImpl(());

impl<M> Component<M> for FindNodeImpl
where
    M: Module + HasComponent<dyn FilterUsers>,
{
    type Interface = dyn FindNode;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let filter: Arc<dyn FilterUsers> = M::build_component(context);
        let filter = filter.upcast();
        Box::new(data::FindNode::new(filter))
    }
}
