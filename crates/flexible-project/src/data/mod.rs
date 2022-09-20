//! Data for the [actix-web](actix_web) backend of the Flexible Project system.

use fp_core::model::UserRole;
use fp_data::data_source::mock::MockUserDataSource;
use fp_data::model::UserData;
use fp_data::repository::user::UserRepository;
use tokio::sync::RwLock;

/// User repository data wrapped with [`RwLock`].
pub type UserRepositoryData = RwLock<UserRepository<MockUserDataSource>>;

/// Creates user repository of the Flexible Project system
/// which uses inner data source implementation.
pub fn create_user_repository() -> UserRepositoryData {
    let user = UserData {
        id: "Hello World".into(),
        name: "tuguzT".to_string(),
        email: Some("timurka.tugushev@gmail.com".to_string()),
        role: UserRole::Administrator,
    };
    let data_source = [user].into_iter().collect();
    let repository = UserRepository::new(data_source);
    RwLock::new(repository)
}
