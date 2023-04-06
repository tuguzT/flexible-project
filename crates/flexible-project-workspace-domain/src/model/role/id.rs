use fp_core::id::{Id, IdFilters};

use super::Role;

/// Type of workspace role identifier.
pub type RoleId = Id<Role>;

/// Filters for workspace role identifiers of the backend.
pub type RoleIdFilters<'a> = IdFilters<'a, Role>;
