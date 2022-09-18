//! Data sources for users of the Flexible Project system.

use crate::data_source::CrudDataSource;
use crate::model::UserData;

/// User data source of the Flexible Project system.
pub trait UserDataSource: CrudDataSource<Item = UserData> {}
