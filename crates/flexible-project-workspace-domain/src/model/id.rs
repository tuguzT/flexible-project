use fp_core::id::{Id, IdFilters};

use super::workspace::Workspace;

/// Type of workspace identifier.
pub type WorkspaceId = Id<Workspace>;

/// Filters for workspace identifiers of the backend.
pub type WorkspaceIdFilters<'a> = IdFilters<'a, Workspace>;
