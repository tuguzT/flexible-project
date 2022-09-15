#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server library.

use fp_core::model::User;
use fp_data::data_source::mock::user::MockUserDataSource;
use fp_data::data_source::user::UserDataSource;
use fp_data::repository::user::UserRepository;
use tokio::sync::RwLock;

pub mod routes;

type MockUserRepository = UserRepository<MockUserDataSource>;

/// Creates user repository and wraps it with [`RwLock`].
///
/// Created repository uses inner data source which is not visible from the outside.
pub fn user_repository() -> RwLock<UserRepository<impl UserDataSource<Item = impl User>>> {
    let data_source = MockUserDataSource::default();
    let repository: MockUserRepository = UserRepository::new(data_source);
    RwLock::new(repository)
}
