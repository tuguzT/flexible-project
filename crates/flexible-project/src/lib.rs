#![warn(missing_docs)]
#![warn(clippy::all)]

//! Flexible Project server library.

use actix_web::web;
use fp_data::data_source::mock::MockUserDataSource;
use fp_data::repository::mock::MockUserRepository;
use fp_data::repository::user::UserRepository;
use tokio::sync::RwLock;

pub mod config;

type RwData<T> = web::Data<RwLock<T>>;

/// Creates user repository of the Flexible Project system
/// which uses inner data source implementation.
pub fn user_repository() -> RwData<impl UserRepository> {
    let data_source = MockUserDataSource::default();
    let repository = MockUserRepository::new(data_source);
    web::Data::new(RwLock::new(repository))
}
