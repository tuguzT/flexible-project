use fp_core::model::UserRole;
use fp_data::data_source::mock::MockUserDataSource;
use fp_data::model::{Id, User};
use fp_data::repository::user::UserRepository;

/// User repository data.
pub type RepositoryData = UserRepository<MockUserDataSource>;

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
        name: "dr3am_b3ast".to_string(),
        email: None,
        role: UserRole::Moderator,
    };
    let data_source = [admin, moderator].into_iter().collect();
    UserRepository::new(data_source)
}
