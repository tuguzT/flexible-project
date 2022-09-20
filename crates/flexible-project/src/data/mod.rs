//! Data for the [actix-web](actix_web) backend of the Flexible Project system.

pub use user::create_repository as create_user_repository;
pub use user::RepositoryData as UserRepositoryData;

mod user;
