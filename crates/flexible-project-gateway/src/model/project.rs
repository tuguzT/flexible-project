//! Project data model of the gateway service.

use async_graphql::{ComplexObject, Enum, InputObject, Object, SimpleObject, ID};
use chrono::{DateTime, Utc};

use super::{
    methodology::{Methodology, MethodologyStage},
    user::User,
};

/// Query object of projects of the Flexible Project system.
#[derive(Debug, Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    /// Filters all projects of the system.
    pub async fn projects(&self, filters: ProjectFilters) -> Vec<Project> {
        let _ = filters;
        None.unwrap()
    }
}

/// Mutation object of projects of the Flexible Project system.
#[derive(Debug, Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    /// Creates new project in the system.
    pub async fn create_project(&self, workspace: ID, name: String) -> Project {
        let _ = (workspace, name);
        None.unwrap()
    }

    /// Updates properties of the project by provided identifier with provided data.
    pub async fn update_project(&self, id: ID, update: UpdateProject) -> Project {
        let _ = (id, update);
        None.unwrap()
    }

    /// Deletes project from the system by provided identifier.
    pub async fn delete_project(&self, id: ID) -> Project {
        let _ = id;
        None.unwrap()
    }
}

/// Project properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[graphql(complex)]
pub struct Project {
    /// Unique identifier of the project.
    pub id: ID,
    /// Name of the project.
    pub name: String,
    /// Description of the project.
    pub description: String,
    /// Visibility of the project.
    pub visibility: ProjectVisibility,
    /// Current stage of methodology used in the project.
    pub current_stage: MethodologyStage,
    /// Members of the project.
    pub members: Vec<ProjectMember>,
    /// Roles of the project.
    pub roles: Vec<ProjectRole>,
    /// Tasks of the project.
    pub tasks: Vec<ProjectTask>,
}

#[ComplexObject]
impl Project {
    /// Methodology used in the project.
    pub async fn methodology(&self) -> Methodology {
        None.unwrap()
    }
}

/// Filters of projects of the Flexible Project system.
#[derive(Debug, InputObject)]
pub struct ProjectFilters {
    /// Identifier filter of the project.
    pub id: Option<ID>,
}

/// Data of the project to update.
#[derive(Debug, InputObject)]
pub struct UpdateProject {
    /// Name of the project to update, if present.
    pub name: Option<String>,
    /// Description of the project to update, if present.
    pub description: Option<String>,
    /// Visibility of the project to update, if present.
    pub visibility: Option<ProjectVisibility>,
}

/// Visibility level of the project of the Flexible Project system.
#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectVisibility {
    /// Workspace is visible for any user which can view the parent workspace.
    Public,
    /// Workspace is only visible for members of this project.
    Private,
}

/// Project member properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectMember {
    /// User which is a member of the project.
    pub user: User,
    /// Role of member in the project.
    pub roles: Vec<ProjectRole>,
}

/// Project role properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectRole {
    /// Unique identifier of the role.
    pub id: ID,
    /// Name of the role.
    pub name: String,
    /// Description of the role.
    pub description: String,
    /// Color of the role.
    pub color: i32,
    /// Set of operations of the role.
    pub operations: Vec<ProjectOperation>,
}

/// Project operation properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectOperation {
    /// Unique identifier of the operation.
    pub id: ID,
    /// Name of the operation.
    pub name: String,
    /// Targets of the operation.
    pub targets: Vec<ID>, // TODO target interface
}

/// Project task properties of the Flexible Project system.
#[derive(Debug, SimpleObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProjectTask {
    /// Unique identifier of the task.
    pub id: ID,
    /// Name of the task.
    pub name: String,
    /// Description of the task.
    pub description: String,
    /// Start date of the task.
    pub start_date: Option<DateTime<Utc>>,
    /// End date of the task.
    pub end_date: Option<DateTime<Utc>>,
    /// If the task is completed.
    pub completed: bool,
}
