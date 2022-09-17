//! Repositories for users of the Flexible Project system.

use crate::model::UserData;
use crate::repository::CrudRepository;

/// CRUD repository for users of the Flexible Project system.
pub trait UserRepository: CrudRepository<Item = UserData> {}
