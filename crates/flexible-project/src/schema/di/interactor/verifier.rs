//! Verifier interactor components and providers for dependency injection.

use std::sync::Arc;

use shaku::{Component, HasComponent, Interface, Module};

use super::token::TokenSecret;

mod core {
    pub use fp_core::use_case::verifier::{
        PasswordVerifier, UserCredentialsVerifier, UserTokenVerifier, UsernameVerifier,
    };
}

mod data {
    pub use fp_data::interactor::verifier::{
        PasswordVerifier, UserCredentialsVerifier, UserTokenVerifier, UsernameVerifier,
    };
}

/// Password verifier interface for dependency injection.
pub trait PasswordVerifier: core::PasswordVerifier + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::PasswordVerifier>;
}

impl<T> PasswordVerifier for T
where
    T: core::PasswordVerifier + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::PasswordVerifier> {
        self
    }
}

/// Password verifier component.
pub struct PasswordVerifierComponent(());

impl<M> Component<M> for PasswordVerifierComponent
where
    M: Module,
{
    type Interface = dyn PasswordVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::<data::PasswordVerifier>::default()
    }
}

/// User credentials verifier interface for dependency injection.
pub trait UserCredentialsVerifier: core::UserCredentialsVerifier + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UserCredentialsVerifier>;
}

impl<T> UserCredentialsVerifier for T
where
    T: core::UserCredentialsVerifier + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UserCredentialsVerifier> {
        self
    }
}

/// User credentials verifier component.
pub struct UserCredentialsVerifierComponent(());

impl<M> Component<M> for UserCredentialsVerifierComponent
where
    M: Module + HasComponent<dyn UsernameVerifier> + HasComponent<dyn PasswordVerifier>,
{
    type Interface = dyn UserCredentialsVerifier;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let username_verifier: Arc<dyn UsernameVerifier> = M::build_component(context);
        let username_verifier = username_verifier.upcast();

        let password_verifier: Arc<dyn PasswordVerifier> = M::build_component(context);
        let password_verifier = password_verifier.upcast();

        let user_credentials_verifier =
            data::UserCredentialsVerifier::new(username_verifier, password_verifier);
        Box::new(user_credentials_verifier)
    }
}

/// User token verifier interface for dependency injection.
pub trait UserTokenVerifier: core::UserTokenVerifier + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UserTokenVerifier>;
}

impl<T> UserTokenVerifier for T
where
    T: core::UserTokenVerifier + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UserTokenVerifier> {
        self
    }
}

/// User token verifier component.
pub struct UserTokenVerifierComponent(());

impl<M> Component<M> for UserTokenVerifierComponent
where
    M: Module + HasComponent<TokenSecret>,
{
    type Interface = dyn UserTokenVerifier;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let secret = M::build_component(context).0.clone();
        Box::new(data::UserTokenVerifier::new(secret))
    }
}

/// Username verifier interface for dependency injection.
pub trait UsernameVerifier: core::UsernameVerifier + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UsernameVerifier>;
}

impl<T> UsernameVerifier for T
where
    T: core::UsernameVerifier + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UsernameVerifier> {
        self
    }
}

/// Username verifier component.
pub struct UsernameVerifierComponent(());

impl<M> Component<M> for UsernameVerifierComponent
where
    M: Module,
{
    type Interface = dyn UsernameVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::<data::UsernameVerifier>::default()
    }
}
