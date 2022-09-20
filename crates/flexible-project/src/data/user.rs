use fp_core::model::UserRole;
use fp_data::data_source::mock::MockUserDataSource;
use fp_data::model::{Id, User};
use fp_data::repository::user::UserRepository;
use tokio::sync::RwLock;

/// User repository data wrapped with [`RwLock`].
pub type RepositoryData = RwLock<UserRepository<MockUserDataSource>>;

/// Creates user repository of the Flexible Project system
/// which uses inner data source implementation.
pub fn create_repository() -> RepositoryData {
    let admin = User {
        id: Id::random(),
        name: "tuguzT".to_string(),
        email: Some("timurka.tugushev@gmail.com".to_string()),
        role: UserRole::Administrator,
    };
    let moderator = User {
        id: Id::random(),
        name: "dr3amb3ast".to_string(),
        email: None,
        role: UserRole::Moderator,
    };
    let data_source = [admin, moderator].into_iter().collect();
    let repository = UserRepository::new(data_source);
    RwLock::new(repository)
}
