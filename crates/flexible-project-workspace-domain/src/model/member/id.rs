use fp_user_domain::model::{UserId, UserIdFilters};

/// Identifier of the user which is a member of the workspace.
pub type MemberId = UserId;

/// Filters for member identifiers of the workspace.
pub type MemberIdFilters<'a> = UserIdFilters<'a>;
