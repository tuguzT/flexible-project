use fp_core::id::{ErasedId, ErasedIdFilters};

/// Identifier of the user which is a member of the workspace.
pub type MemberId = ErasedId;

/// Filters for member identifiers of the workspace.
pub type MemberIdFilters<'a> = ErasedIdFilters<'a>;
