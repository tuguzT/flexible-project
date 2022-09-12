//! Data sources for users of the Flexible Project system.

use fp_core::model::User;

use crate::data_source::CrudDataSource;

/// CRUD data source for users of the Flexible Project system.
pub trait UserDataSource: CrudDataSource
where
    Self::Item: User,
{
}
