//! Dependency injection of the Flexible Project schema.

use shaku::module;

pub mod data_source;
pub mod interactor;

module! {
    pub(in crate::schema) SchemaModule {
        components = [
            data_source::client::DatabaseUrl,
            data_source::client::ClientComponent,
            data_source::user::UserDataSourceComponent,

            interactor::hasher::SharedPasswordHasher,
            interactor::hasher::PasswordHasherComponent,
            interactor::hasher::PasswordHashVerifierComponent,
            interactor::id::IdGeneratorComponent,
            interactor::node::FindNodeComponent,
            interactor::user::CurrentUserComponent,
            interactor::user::DeleteUserComponent,
            interactor::user::FilterUsersComponent,
            interactor::user::UserTokenGeneratorComponent,
            interactor::user::SignInComponent,
            interactor::user::SignUpComponent,
            interactor::user::UpdateUserDisplayNameComponent,
            interactor::user::UpdateUserEmailComponent,
            interactor::user::GrantUserRoleComponent,
            interactor::user::UpdateUsernameComponent,
            interactor::user::UpdateUserPasswordComponent,
            interactor::token::TokenSecret,
            interactor::verifier::PasswordVerifierComponent,
            interactor::verifier::UserCredentialsVerifierComponent,
            interactor::verifier::UserTokenVerifierComponent,
            interactor::verifier::UsernameVerifierComponent,
        ],
        providers = [],
    }
}
