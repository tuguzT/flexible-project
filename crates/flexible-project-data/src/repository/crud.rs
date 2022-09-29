use fp_core::model::Node;

use crate::repository::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};

/// Repository type which combines together all operations
/// provided by [`repository::ops`](crate::repository::ops) module.
pub trait CrudRepository: Clear + Delete + DeleteById + ReadAll + ReadById + Save
where
    Self::Item: Node,
{
}

impl<T> CrudRepository for T
where
    T: Clear + Delete + DeleteById + ReadAll + ReadById + Save,
    T::Item: Node,
{
}
