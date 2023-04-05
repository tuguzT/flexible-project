pub use self::{
    level::{RoleAccessLevel, RoleAccessLevelFilters, RoleUpdateOperations},
    operation::{RoleUpdateOperation, RoleUpdateOperationFilters},
    scope::{RoleUpdateOperationScope, RoleUpdateOperationScopeFilters},
};

mod level;
mod operation;
mod scope;
