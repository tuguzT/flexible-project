//! Data sources for users of the Flexible Project system.

use crate::data_source::CrudDataSource;
use crate::model::User;

/// User data source type of the Flexible Project system.
pub trait UserDataSource: CrudDataSource<Item = User> {}
