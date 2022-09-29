use fp_core::model::Node;

use crate::data_source::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};

/// Data source type which combines together all operations
/// provided by [`data_source::ops`](crate::data_source::ops) module.
pub trait CrudDataSource: Clear + Delete + DeleteById + ReadAll + ReadById + Save
where
    Self::Item: Node,
{
}

impl<T> CrudDataSource for T
where
    T: Clear + Delete + DeleteById + ReadAll + ReadById + Save,
    T::Item: Node,
{
}
