use std::sync::Arc;

use shaku::{Component, Interface, Module};

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

pub trait PasswordVerifier: core::PasswordVerifier + Interface {}
impl<T> PasswordVerifier for T where T: ?Sized + core::PasswordVerifier + Interface {}

pub struct PasswordVerifierImpl(data::PasswordVerifier);

impl<M> Component<M> for PasswordVerifierImpl
where
    M: Module,
{
    type Interface = dyn PasswordVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::PasswordVerifier::default())
    }
}

pub trait UserCredentialsVerifier: core::UserCredentialsVerifier + Interface {
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

pub struct UserCredentialsVerifierImpl(data::UserCredentialsVerifier);

impl<M> Component<M> for UserCredentialsVerifierImpl
where
    M: Module,
{
    type Interface = dyn UserCredentialsVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::UserCredentialsVerifier::default())
    }
}

pub trait UserTokenVerifier: core::UserTokenVerifier + Interface {
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

pub struct UserTokenVerifierImpl(data::UserTokenVerifier);

impl<M> Component<M> for UserTokenVerifierImpl
where
    M: Module,
{
    type Interface = dyn UserTokenVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::UserTokenVerifier::default())
    }
}

pub trait UsernameVerifier: core::UsernameVerifier + Interface {}
impl<T> UsernameVerifier for T where T: ?Sized + core::UsernameVerifier + Interface {}

pub struct UsernameVerifierImpl(data::UsernameVerifier);

impl<M> Component<M> for UsernameVerifierImpl
where
    M: Module,
{
    type Interface = dyn UsernameVerifier;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        Box::new(data::UsernameVerifier::default())
    }
}
