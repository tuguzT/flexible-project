use fp_core::id::{Id, IdFilters};

use super::user::User;

/// Type of user identifier.
pub type UserId = Id<User>;

/// Filters for user identifiers of the backend.
pub type UserIdFilters = IdFilters<User>;
