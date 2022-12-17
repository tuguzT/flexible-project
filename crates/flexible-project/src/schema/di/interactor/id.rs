//! Identifier interactor components and providers for dependency injection.

use std::sync::Arc;

use shaku::{Component, Interface, Module};

mod core {
    pub use fp_core::use_case::id::IdGenerator;
}

mod data {
    pub use fp_data::interactor::id::IdGenerator;
}

/// Identifier generator interface for dependency injection.
pub trait IdGenerator: core::IdGenerator + Interface {
    /// Upcasts to the base trait.
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

/// Identifier generator component.
pub struct IdGeneratorImpl(());

impl<M> Component<M> for IdGeneratorImpl
where
    M: Module,
{
    type Interface = dyn IdGenerator;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::<data::IdGenerator>::default()
    }
}
