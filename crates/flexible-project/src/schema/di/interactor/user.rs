//! User interactor components and providers for dependency injection.

use std::sync::Arc;

use shaku::{Component, HasComponent, Interface, Module};

use crate::schema::di::{
    data_source::user::UserDataSource,
    interactor::{
        hasher::{PasswordHashVerifier, PasswordHasher},
        id::IdGenerator,
        verifier::{
            PasswordVerifier, UserCredentialsVerifier, UserTokenVerifier, UsernameVerifier,
        },
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
        CurrentUser, DeleteUser, FilterUsers, GrantUserRole, SignIn, SignUp, UpdateUserDisplayName,
        UpdateUserEmail, UpdateUserPassword, UpdateUsername, UserTokenGenerator,
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
pub struct CurrentUserComponent(());

impl<M> Component<M> for CurrentUserComponent
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

        let current_user = data::CurrentUser::new(repository, token_verifier);
        Box::new(current_user)
    }
}

/// Delete user interface for dependency injection.
pub trait DeleteUser: core::DeleteUser + Interface {}
impl<T> DeleteUser for T where T: ?Sized + core::DeleteUser + Interface {}

/// Delete user component.
pub struct DeleteUserComponent(());

impl<M> Component<M> for DeleteUserComponent
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

        let delete_user = data::DeleteUser::new(repository, current_user);
        Box::new(delete_user)
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
pub struct FilterUsersComponent(());

impl<M> Component<M> for FilterUsersComponent
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

        let filter_users = data::FilterUsers::new(repository);
        Box::new(filter_users)
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
pub struct UserTokenGeneratorComponent(());

impl<M> Component<M> for UserTokenGeneratorComponent
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

        let user_token_generator = data::UserTokenGenerator::new(secret);
        Box::new(user_token_generator)
    }
}

/// Sign in interface for dependency injection.
pub trait SignIn: core::SignIn + Interface {}
impl<T> SignIn for T where T: ?Sized + core::SignIn + Interface {}

/// Sign in component.
pub struct SignInComponent(());

impl<M> Component<M> for SignInComponent
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
pub struct SignUpComponent(());

impl<M> Component<M> for SignUpComponent
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

/// Update user display name interface for dependency injection.
pub trait UpdateUserDisplayName: core::UpdateUserDisplayName + Interface {}
impl<T> UpdateUserDisplayName for T where T: core::UpdateUserDisplayName + Interface {}

/// Update user display name component.
pub struct UpdateUserDisplayNameComponent(());

impl<M> Component<M> for UpdateUserDisplayNameComponent
where
    M: Module + HasComponent<dyn UserDataSource> + HasComponent<dyn CurrentUser>,
{
    type Interface = dyn UpdateUserDisplayName;

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

        let update_user_display_name = data::UpdateUserDisplayName::new(repository, current_user);
        Box::new(update_user_display_name)
    }
}

/// Update user email interface for dependency injection.
pub trait UpdateUserEmail: core::UpdateUserEmail + Interface {}
impl<T> UpdateUserEmail for T where T: core::UpdateUserEmail + Interface {}

/// Update user email component.
pub struct UpdateUserEmailComponent(());

impl<M> Component<M> for UpdateUserEmailComponent
where
    M: Module + HasComponent<dyn UserDataSource> + HasComponent<dyn CurrentUser>,
{
    type Interface = dyn UpdateUserEmail;

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

        let update_user_email = data::UpdateUserEmail::new(repository, current_user);
        Box::new(update_user_email)
    }
}

/// Grant user role interface for dependency injection.
pub trait GrantUserRole: core::GrantUserRole + Interface {}
impl<T> GrantUserRole for T where T: core::GrantUserRole + Interface {}

/// Grant user role component.
pub struct GrantUserRoleComponent(());

impl<M> Component<M> for GrantUserRoleComponent
where
    M: Module + HasComponent<dyn UserDataSource> + HasComponent<dyn CurrentUser>,
{
    type Interface = dyn GrantUserRole;

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

        let grant_user_role = data::GrantUserRole::new(repository, current_user);
        Box::new(grant_user_role)
    }
}

/// Update username interface for dependency injection.
pub trait UpdateUsername: core::UpdateUsername + Interface {}
impl<T> UpdateUsername for T where T: core::UpdateUsername + Interface {}

/// Update username component.
pub struct UpdateUsernameComponent(());

impl<M> Component<M> for UpdateUsernameComponent
where
    M: Module
        + HasComponent<dyn UserDataSource>
        + HasComponent<dyn CurrentUser>
        + HasComponent<dyn UsernameVerifier>,
{
    type Interface = dyn UpdateUsername;

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

        let username_verifier: Arc<dyn UsernameVerifier> = M::build_component(context);
        let username_verifier = username_verifier.upcast();

        let update_username =
            data::UpdateUsername::new(repository, username_verifier, current_user);
        Box::new(update_username)
    }
}

/// Update user password interface for dependency injection.
pub trait UpdateUserPassword: core::UpdateUserPassword + Interface {}
impl<T> UpdateUserPassword for T where T: core::UpdateUserPassword + Interface {}

/// Update user password component.
pub struct UpdateUserPasswordComponent(());

impl<M> Component<M> for UpdateUserPasswordComponent
where
    M: Module
        + HasComponent<dyn UserDataSource>
        + HasComponent<dyn CurrentUser>
        + HasComponent<dyn PasswordVerifier>
        + HasComponent<dyn PasswordHasher>
        + HasComponent<dyn PasswordHashVerifier>,
{
    type Interface = dyn UpdateUserPassword;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source: Arc<dyn UserDataSource> = M::build_component(context);
        let data_source = data_source.upcast();
        let repository = data::UserRepository(data_source);

        let password_verifier: Arc<dyn PasswordVerifier> = M::build_component(context);
        let password_verifier = password_verifier.upcast();

        let password_hasher: Arc<dyn PasswordHasher> = M::build_component(context);
        let password_hasher = password_hasher.upcast();

        let password_hash_verifier: Arc<dyn PasswordHashVerifier> = M::build_component(context);
        let password_hash_verifier = password_hash_verifier.upcast();

        let current_user: Arc<dyn CurrentUser> = M::build_component(context);
        let current_user = current_user.upcast();

        let update_user_password = data::UpdateUserPassword::new(
            repository,
            password_verifier,
            password_hasher,
            password_hash_verifier,
            current_user,
        );
        Box::new(update_user_password)
    }
}
