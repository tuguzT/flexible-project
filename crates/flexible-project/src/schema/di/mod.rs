use shaku::module;

pub mod data_source;
pub mod interactor;

module! {
    pub SchemaModule {
        components = [
            data_source::client::ClientImpl,
            data_source::user::UserDataSourceImpl,

            interactor::hasher::PasswordHasherImpl, interactor::hasher::PasswordHashVerifierImpl,
            interactor::id::IdGeneratorImpl,
            interactor::node::FindNodeImpl,
            interactor::user::CurrentUserImpl, interactor::user::DeleteUserImpl,
            interactor::user::FilterUsersImpl, interactor::user::UserTokenGeneratorImpl,
            interactor::user::SignInImpl, interactor::user::SignUpImpl,
            interactor::verifier::PasswordVerifierImpl, interactor::verifier::UserCredentialsVerifierImpl,
            interactor::verifier::UserTokenVerifierImpl, interactor::verifier::UsernameVerifierImpl,
        ],
        providers = [],
    }
}
