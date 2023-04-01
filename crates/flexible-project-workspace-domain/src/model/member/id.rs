use fp_user_domain::model::{UserId, UserIdFilters};

/// Type of workspace member identifier.
pub type MemberId = UserId;

/// Filters for workspace member identifiers of the backend.
pub type MemberIdFilters<'a> = UserIdFilters<'a>;
