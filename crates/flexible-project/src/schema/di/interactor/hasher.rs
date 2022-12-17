//! Hasher interactor components and providers for dependency injection.

use std::sync::Arc;

use shaku::{Component, HasComponent, Interface, Module};

mod core {
    pub use fp_core::use_case::hasher::{PasswordHashVerifier, PasswordHasher};
}

mod data {
    pub use fp_data::interactor::hasher::PasswordHasher;
}

/// Shared password hasher component.
pub struct SharedPasswordHasher(());

impl<M> Component<M> for SharedPasswordHasher
where
    M: Module,
{
    type Interface = data::PasswordHasher;

    type Parameters = data::PasswordHasher;

    fn build(
        _: &mut shaku::ModuleBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        Box::new(params)
    }
}

/// Password hasher interface for dependency injection.
pub trait PasswordHasher: core::PasswordHasher + Interface {
    /// Upcasts to the base trait.
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

/// Password hasher component.
pub struct PasswordHasherImpl(());

impl<M> Component<M> for PasswordHasherImpl
where
    M: Module + HasComponent<data::PasswordHasher>,
{
    type Interface = dyn PasswordHasher;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let password_hasher = M::build_component(context);
        Box::new(password_hasher)
    }
}

/// Password verifier interface for dependency injection.
pub trait PasswordHashVerifier: core::PasswordHashVerifier + Interface {
    /// Upcasts to the base trait.
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

/// Password hash verifier component.
pub struct PasswordHashVerifierImpl(());

impl<M> Component<M> for PasswordHashVerifierImpl
where
    M: Module + HasComponent<data::PasswordHasher>,
{
    type Interface = dyn PasswordHashVerifier;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let password_hash_verifier = M::build_component(context);
        Box::new(password_hash_verifier)
    }
}
