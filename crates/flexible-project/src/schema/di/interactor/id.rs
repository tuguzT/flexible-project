use std::sync::Arc;

use shaku::{Component, Interface, Module};

mod core {
    pub use fp_core::use_case::id::IdGenerator;
}

mod data {
    pub use fp_data::interactor::id::IdGenerator;
}

pub trait IdGenerator: core::IdGenerator + Interface {
    fn upcast(self: Arc<Self>) -> Arc<dyn core::IdGenerator>;
}

impl<T> IdGenerator for T
where
    T: core::IdGenerator + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::IdGenerator> {
        self
    }
}

pub struct IdGeneratorImpl(());

impl<M> Component<M> for IdGeneratorImpl
where
    M: Module,
{
    type Interface = dyn IdGenerator;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::IdGenerator::default())
    }
}
