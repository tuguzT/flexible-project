use crate::repository::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};

/// Trait for repository which combines together all operations
/// provided by [`repository::ops`](crate::repository::ops) module.
pub trait CrudRepository: Clear + Delete + DeleteById + ReadAll + ReadById + Save {}

impl<T> CrudRepository for T where T: Clear + Delete + DeleteById + ReadAll + ReadById + Save {}
