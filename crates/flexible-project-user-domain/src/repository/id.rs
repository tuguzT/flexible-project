use fp_core::id::GenerateId;

use crate::model::User;

/// Generator of unique user identifiers.
pub trait GenerateUserId: GenerateId<User> {}
impl<T> GenerateUserId for T where T: GenerateId<User> + ?Sized {}
