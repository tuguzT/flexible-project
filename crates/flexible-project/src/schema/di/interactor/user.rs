//! User interactor components and providers for dependency injection.

use std::sync::Arc;

use shaku::{Component, HasComponent, Interface, Module};

use crate::schema::di::{
    data_source::user::UserDataSource,
    interactor::{
        hasher::{PasswordHashVerifier, PasswordHasher},
        id::IdGenerator,
        verifier::{UserCredentialsVerifier, UserTokenVerifier},
    },
};

use super::token::TokenSecret;

mod core {
    pub use fp_core::use_case::user::{
        CurrentUser, DeleteUser, FilterUsers, GrantUserRole, SignIn, SignUp, UpdateUserDisplayName,
        UpdateUserEmail, UpdateUserPassword, UpdateUsername, UserTokenGenerator,
    };
}

mod data {
    pub use fp_data::interactor::user::{
        CurrentUser, DeleteUser, FilterUsers, SignIn, SignUp, UserTokenGenerator,
    };
    pub use fp_data::repository::user::UserRepository;
}

/// Current user interface for dependency injection.
pub trait CurrentUser: core::CurrentUser + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::CurrentUser>;
}

impl<T> CurrentUser for T
where
    T: core::CurrentUser + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::CurrentUser> {
        self
    }
}

/// Current user component.
pub struct CurrentUserImpl(());

impl<M> Component<M> for CurrentUserImpl
where
    M: Module + HasComponent<dyn UserDataSource> + HasComponent<dyn UserTokenVerifier>,
{
    type Interface = dyn CurrentUser;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source: Arc<dyn UserDataSource> = M::build_component(context);
        let data_source = data_source.upcast();
        let repository = data::UserRepository(data_source);

        let token_verifier: Arc<dyn UserTokenVerifier> = M::build_component(context);
        let token_verifier = token_verifier.upcast();

        Box::new(data::CurrentUser::new(repository, token_verifier))
    }
}

/// Delete user interface for dependency injection.
pub trait DeleteUser: core::DeleteUser + Interface {}
impl<T> DeleteUser for T where T: ?Sized + core::DeleteUser + Interface {}

/// Delete user component.
pub struct DeleteUserImpl(());

impl<M> Component<M> for DeleteUserImpl
where
    M: Module + HasComponent<dyn UserDataSource> + HasComponent<dyn CurrentUser>,
{
    type Interface = dyn DeleteUser;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source: Arc<dyn UserDataSource> = M::build_component(context);
        let data_source = data_source.upcast();
        let repository = data::UserRepository(data_source);

        let current_user: Arc<dyn CurrentUser> = M::build_component(context);
        let current_user = current_user.upcast();

        Box::new(data::DeleteUser::new(repository, current_user))
    }
}

/// Filter users interface for dependency injection.
pub trait FilterUsers: core::FilterUsers + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::FilterUsers>;
}

impl<T> FilterUsers for T
where
    T: core::FilterUsers + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::FilterUsers> {
        self
    }
}

/// Filter users component.
pub struct FilterUsersImpl(());

impl<M> Component<M> for FilterUsersImpl
where
    M: Module + HasComponent<dyn UserDataSource>,
{
    type Interface = dyn FilterUsers;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source: Arc<dyn UserDataSource> = M::build_component(context);
        let data_source = data_source.upcast();
        let repository = data::UserRepository(data_source);

        Box::new(data::FilterUsers::new(repository))
    }
}

/// User token generator interface for dependency injection.
pub trait UserTokenGenerator: core::UserTokenGenerator + Interface {
    /// Upcasts to the base trait.
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UserTokenGenerator>;
}

impl<T> UserTokenGenerator for T
where
    T: core::UserTokenGenerator + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn core::UserTokenGenerator> {
        self
    }
}

/// User token generator component.
pub struct UserTokenGeneratorImpl(());

impl<M> Component<M> for UserTokenGeneratorImpl
where
    M: Module + HasComponent<TokenSecret>,
{
    type Interface = dyn UserTokenGenerator;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let secret = M::build_component(context).0.clone();
        Box::new(data::UserTokenGenerator::new(secret))
    }
}

/// Sign in interface for dependency injection.
pub trait SignIn: core::SignIn + Interface {}
impl<T> SignIn for T where T: ?Sized + core::SignIn + Interface {}

/// Sign in component.
pub struct SignInImpl(());

impl<M> Component<M> for SignInImpl
where
    M: Module
        + HasComponent<dyn UserDataSource>
        + HasComponent<dyn PasswordHashVerifier>
        + HasComponent<dyn UserTokenGenerator>
        + HasComponent<dyn UserCredentialsVerifier>,
{
    type Interface = dyn SignIn;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source: Arc<dyn UserDataSource> = M::build_component(context);
        let data_source = data_source.upcast();
        let repository = data::UserRepository(data_source);

        let password_hash_verifier: Arc<dyn PasswordHashVerifier> = M::build_component(context);
        let password_hash_verifier = password_hash_verifier.upcast();

        let credentials_verifier: Arc<dyn UserCredentialsVerifier> = M::build_component(context);
        let credentials_verifier = credentials_verifier.upcast();

        let token_generator: Arc<dyn UserTokenGenerator> = M::build_component(context);
        let token_generator = token_generator.upcast();

        let sign_in = data::SignIn::new(
            repository,
            password_hash_verifier,
            credentials_verifier,
            token_generator,
        );
        Box::new(sign_in)
    }
}

/// Sign up interface for dependency injection.
pub trait SignUp: core::SignUp + Interface {}
impl<T> SignUp for T where T: ?Sized + core::SignUp + Interface {}

/// Sign up component.
pub struct SignUpImpl(());

impl<M> Component<M> for SignUpImpl
where
    M: Module
        + HasComponent<dyn UserDataSource>
        + HasComponent<dyn PasswordHasher>
        + HasComponent<dyn UserTokenGenerator>
        + HasComponent<dyn IdGenerator>
        + HasComponent<dyn UserCredentialsVerifier>,
{
    type Interface = dyn SignUp;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source: Arc<dyn UserDataSource> = M::build_component(context);
        let data_source = data_source.upcast();
        let repository = data::UserRepository(data_source);

        let password_hasher: Arc<dyn PasswordHasher> = M::build_component(context);
        let password_hasher = password_hasher.upcast();

        let credentials_verifier: Arc<dyn UserCredentialsVerifier> = M::build_component(context);
        let credentials_verifier = credentials_verifier.upcast();

        let id_generator: Arc<dyn IdGenerator> = M::build_component(context);
        let id_generator = id_generator.upcast();

        let token_generator: Arc<dyn UserTokenGenerator> = M::build_component(context);
        let token_generator = token_generator.upcast();

        let sign_up = data::SignUp::new(
            repository,
            password_hasher,
            credentials_verifier,
            id_generator,
            token_generator,
        );
        Box::new(sign_up)
    }
}
