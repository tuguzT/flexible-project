//! Workspace definitions and utilities for the Flexible Project system model.

#![allow(missing_docs)]

use derive_more::{IsVariant, Unwrap};
use typed_builder::TypedBuilder;

use super::{
    id::{Id, IdFilters},
    project::Project,
    user::User,
};

/// Type of [workspace](Workspace) identifier.
pub type WorkspaceId = Id<Workspace>;

/// Workspace of the Flexible Project system
/// is a collection of [projects](Project)
/// with [members](WorkspaceMember) which have access to these projects.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Workspace {
    /// Identifier of the workspace.
    pub id: Id<Self>,
    /// Name of the workspace.
    pub name: String,
    /// [Members](WorkspaceMember) of the workspace.
    pub members: Vec<WorkspaceMember>,
    /// Identifiers of [projects](Project) of the workspace.
    pub projects: Vec<Id<Project>>,
}

/// Member of the [workspace](Workspace) of the Flexible Project system.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkspaceMember {
    /// Identifier of the workspace member.
    pub id: Id<User>,
    /// Role of the workspace member.
    pub role: WorkspaceMemberRole,
}

/// Role of the workspace [member](WorkspaceMember) of the Flexible Project system.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, IsVariant, Unwrap)]
pub enum WorkspaceMemberRole {
    /// Ordinary member of the workspace.
    #[default]
    Member,
    /// Administrator of the workspace.
    Administrator,
    /// Owner (usually creator) of the workspace.
    Owner,
}

/// Filters to be applied on workspace search process.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct WorkspaceFilters {
    /// Workspace identifier filters.
    pub id: Option<IdFilters<Workspace>>,
    // TODO fill with filters
}
