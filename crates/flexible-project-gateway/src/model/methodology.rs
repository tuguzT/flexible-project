//! Methodology data model of the gateway service.

#![allow(missing_docs)]

use async_graphql::{
    ComplexObject, Enum, InputObject, Interface, Object, OneofObject, SimpleObject, ID,
};
use chrono::Duration;

use super::{user::User, workspace::Workspace};

/// Query object of methodologies of the Flexible Project system.
#[derive(Debug, Default)]
pub struct MethodologyQuery;

#[Object]
impl MethodologyQuery {
    /// Filters all methodologies of the system.
    pub async fn methodologies(&self, filters: MethodologyFilters) -> Vec<Methodology> {
        let _ = filters;
        None.unwrap()
    }
}

/// Mutation object of methodologies of the Flexible Project system.
#[derive(Debug, Default)]
pub struct MethodologyMutation;

#[Object]
impl MethodologyMutation {
    /// Creates new methodology in the system.
    pub async fn create_methodology(
        &self,
        owner: MethodologyOwnerInput,
        name: String,
    ) -> Methodology {
        let _ = (owner, name);
        None.unwrap()
    }

    /// Updates properties of the methodology by provided identifier with provided data.
    pub async fn update_methodology(&self, id: ID, update: UpdateMethodology) -> Methodology {
        let _ = (id, update);
        None.unwrap()
    }

    /// Deletes methodology from the system by provided identifier.
    pub async fn delete_methodology(&self, id: ID) -> Methodology {
        let _ = id;
        None.unwrap()
    }
}

/// Methodology properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Methodology {
    /// Unique identifier of the methodology.
    pub id: ID,
    /// Owner of the methodology.
    pub owner: MethodologyOwner,
    /// Name of the methodology.
    pub name: String,
    /// Description of the methodology.
    pub description: String,
    /// Visibility of the methodology.
    pub visibility: MethodologyVisibility,
    /// Set of roles of the methodology.
    pub roles: Vec<MethodologyRole>,
    /// Set of stages of the methodology.
    pub stages: Vec<MethodologyStage>,
    /// Set of stage links of the methodology.
    pub stage_links: Vec<MethodologyStageLink>,
}

/// Filters of methodology of the Flexible Project system.
#[derive(Debug, InputObject)]
pub struct MethodologyFilters {
    /// Identifier filter of the methodology.
    pub id: Option<ID>,
}

/// Data of the methodology to update.
#[derive(Debug, InputObject)]
pub struct UpdateMethodology {
    /// Owner of the methodology to update, if exists.
    pub owner: Option<MethodologyOwnerInput>,
    /// Name of the methodology to update, if exists.
    pub name: Option<String>,
    /// Description of the methodology to update, if exists.
    pub description: Option<String>,
    /// Visibility of the methodology to update, if exists.
    pub visibility: Option<MethodologyVisibility>,
}

/// Visibility level of the methodology of the Flexible Project system.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodologyVisibility {
    /// Methodology is visible outside of the owner scope.
    Public,
    /// Methodology is visible only for the owner.
    Private,
}

/// Owner of the methodology: either user, workspace of specific project.
#[derive(Debug, Interface, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[graphql(field(name = "id", type = "&ID", desc = "Unique identifier of the object."))]
pub enum MethodologyOwner {
    /// Methodology is owned by the user.
    User(User),
    /// Methodology is owned by the workspace.
    Workspace(Workspace),
    // TODO Project(Project),
}

/// Owner of the methodology: either user, workspace of specific project.
#[derive(Debug, OneofObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MethodologyOwnerInput {
    /// Methodology is owned by the user.
    User(ID),
    /// Methodology is owned by the workspace.
    Workspace(ID),
    /// Methodology is owned by the project.
    Project(ID),
}

/// Methodology role properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodologyRole {
    /// Unique identifier of the role.
    pub id: ID,
    /// Name of the role.
    pub name: String,
    /// Description of the role.
    pub description: String,
    /// Color of the role.
    pub color: i32,
    /// Set of operation of the role applied to the methodology.
    pub operations: Vec<MethodologyOperation>,
}

/// Methodology operation properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodologyOperation {
    /// Unique identifier of the operation.
    pub id: ID,
    /// Name of the operation.
    pub name: String,
    /// Targets of the operation.
    pub targets: Vec<ID>, // TODO target interface
}

/// Methodology stage properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[graphql(complex)]
pub struct MethodologyStage {
    /// Unique identifier of the stage.
    pub id: ID,
    /// Name of the stage.
    pub name: String,
    /// Description of the stage.
    pub description: String,
    /// Duration of the stage.
    pub duration: Duration,
}

#[ComplexObject]
impl MethodologyStage {
    /// Previous stage of this stage, if exists.
    pub async fn previous_stage(&self) -> Option<MethodologyStage> {
        None
    }

    /// Next stage of this stage, if exists.
    pub async fn next_stage(&self) -> Option<MethodologyStage> {
        None
    }
}

/// Methodology stage link properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodologyStageLink {
    /// Current stage of the link.
    pub current_stage: MethodologyStage,
    /// Next stage of the link.
    pub next_stage: MethodologyStage,
    /// Name of the link.
    pub name: String,
    /// Set of transition conditions of the link.
    pub transition_conditions: Vec<MethodologyStageTransitionCondition>,
}

/// Methodology stage transition condition properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodologyStageTransitionCondition {
    /// Unique identifier of the transition condition.
    pub id: ID,
    // TODO more fields
}
