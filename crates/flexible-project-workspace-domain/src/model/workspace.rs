use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use fp_core::filter::Filter;
use typed_builder::TypedBuilder;

use super::{
    id::{WorkspaceId, WorkspaceIdFilters},
    member::MemberId,
    name::{Name, NameFilters},
};

/// Model of workspace in the system.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Unique identifier of the workspace.
    pub id: WorkspaceId,
    /// Data of the workspace.
    pub data: WorkspaceData,
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Workspace {}

impl Hash for Workspace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Data of the workspace in the system.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct WorkspaceData {
    /// Name of the workspace.
    pub name: Name,
    /// Members of the workspace.
    pub members: Vec<MemberId>,
}

/// Filters for workspaces of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct WorkspaceFilters<'a> {
    /// Workspace identifier filters.
    pub id: Option<WorkspaceIdFilters<'a>>,
    /// Workspace data filters.
    pub data: Option<WorkspaceDataFilters<'a>>,
}

impl Filter for WorkspaceFilters<'_> {
    type Input = Workspace;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self {
            id: id_filter,
            data: data_filter,
        } = self;
        let Workspace { id, data } = input.borrow();
        id_filter.satisfies(id) && data_filter.satisfies(data)
    }
}

/// Filters for workspace data of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct WorkspaceDataFilters<'a> {
    /// Workspace name filters.
    pub name: Option<NameFilters<'a>>,
    // TODO workspace members filters
}

impl Filter for WorkspaceDataFilters<'_> {
    type Input = WorkspaceData;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self { name: name_filter } = self;
        let WorkspaceData { name, .. } = input.borrow();
        name_filter.satisfies(name)
    }
}
