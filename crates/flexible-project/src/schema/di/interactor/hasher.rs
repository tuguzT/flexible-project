use std::sync::Arc;

use shaku::{Component, Interface, Module};

mod core {
    pub use fp_core::use_case::hasher::{PasswordHashVerifier, PasswordHasher};
}

mod data {
    pub use fp_data::interactor::hasher::PasswordHasher;
}

pub trait PasswordHasher: core::PasswordHasher + Interface {
    fn upcast(self: Arc<Self>) -> Arc<dyn core::PasswordHasher>;
}

impl<T> PasswordHasher for T
where
    T: core::PasswordHasher + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::PasswordHasher> {
        self
    }
}

pub struct PasswordHasherImpl(data::PasswordHasher);

impl<M> Component<M> for PasswordHasherImpl
where
    M: Module,
{
    type Interface = dyn PasswordHasher;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::PasswordHasher::default())
    }
}

pub trait PasswordHashVerifier: core::PasswordHashVerifier + Interface {
    fn upcast(self: Arc<Self>) -> Arc<dyn core::PasswordHashVerifier>;
}

impl<T> PasswordHashVerifier for T
where
    T: core::PasswordHashVerifier + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::PasswordHashVerifier> {
        self
    }
}

pub struct PasswordHashVerifierImpl(data::PasswordHasher);

impl<M> Component<M> for PasswordHashVerifierImpl
where
    M: Module,
{
    type Interface = dyn PasswordHashVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::PasswordHasher::default())
    }
}
