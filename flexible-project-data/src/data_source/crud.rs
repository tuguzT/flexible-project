use crate::data_source::ops::{Clear, Delete, DeleteById, ReadAll, ReadById, Save};

/// Trait for data storage which combines together all operations
/// provided by [`data_source::ops`](crate::data_source::ops) module.
pub trait CrudDataSource: Clear + Delete + DeleteById + ReadAll + ReadById + Save {}

impl<T> CrudDataSource for T where T: Clear + Delete + DeleteById + ReadAll + ReadById + Save {}
