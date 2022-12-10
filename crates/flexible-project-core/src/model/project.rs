//! Project definitions and utilities for the Flexible Project system model.

#![allow(missing_docs)]

use derive_more::{IsVariant, Unwrap};

use super::{id::Id, stage::Stage, user::User};

/// Type of [project](Project) identifier.
pub type ProjectId = Id<Project>;

/// Project of the Flexible Project system
/// is a collection of its own [stages](Stage)
/// with [members](ProjectMember) which have access to this project.
///
/// Project is a part of some [workspace](super::workspace::Workspace).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Project {
    /// Identifier of the project.
    pub id: Id<Self>,
    /// Name of the project.
    pub name: String,
    /// [Members](ProjectMember) of the project.
    pub members: Vec<ProjectMember>,
    /// Identifiers of [stages](Stage) of the project.
    pub stages: Vec<Id<Stage>>,
}

/// Member of the [project](Project) of the Flexible Project system.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectMember {
    /// Identifier of the project member.
    pub id: Id<User>,
    /// Role of the project member.
    pub role: ProjectMemberRole,
}

/// Role of the project [member](ProjectMember) of the Flexible Project system.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, IsVariant, Unwrap)]
pub enum ProjectMemberRole {
    /// Ordinary member of the project.
    #[default]
    Member,
    /// Administrator of the project.
    Administrator,
    /// Owner (usually creator) of the project.
    Owner,
}
