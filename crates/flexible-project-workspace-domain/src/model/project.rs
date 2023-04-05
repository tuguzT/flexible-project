use fp_core::id::{ErasedId, ErasedIdFilters};

/// Type of project identifier.
pub type ProjectId = ErasedId;

/// Filters for project identifiers of the backend.
pub type ProjectIdFilters<'a> = ErasedIdFilters<'a>;
