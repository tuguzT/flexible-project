use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    mem::discriminant,
};

use fp_filter::{Equal, Filter, In, NotEqual, NotIn};
use indexmap::IndexSet;

/// Scope of role update operation of the workspace.
#[derive(Debug, Clone)]
pub enum RoleUpdateOperationScope<Target> {
    /// Operation is applicable only for specific targets.
    Restricted {
        /// Set of targets for the scope.
        ///
        /// Operation can be applied only on items of this set.
        allowed: IndexSet<Target>,
    },
    /// Operation is applicable for all targets.
    All {
        /// Set of exceptions for the scope.
        ///
        /// Items of this set are immune to scoped operation.
        exceptions: IndexSet<Target>,
    },
}

impl<Target> PartialEq for RoleUpdateOperationScope<Target>
where
    Target: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        use RoleUpdateOperationScope::*;

        match (self, other) {
            (Restricted { allowed: left }, Restricted { allowed: right }) => left == right,
            (All { exceptions: left }, All { exceptions: right }) => left == right,
            _ => false,
        }
    }
}

impl<Target> Eq for RoleUpdateOperationScope<Target> where Target: Eq + Hash {}

/// This implementation is correct only if targets were not removed from the set,
/// otherwise their order could change.
// FIXME see https://github.com/bluss/indexmap/issues/135
impl<Target> Hash for RoleUpdateOperationScope<Target>
where
    Target: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        // hash discriminant to differentiate between objects with the same set of targets
        discriminant(self).hash(state);
        // hash all the targets to differentiate between objects with the same variant
        let targets = match self {
            Self::Restricted { allowed } => allowed.into_iter(),
            Self::All { exceptions } => exceptions.into_iter(),
        };
        for target in targets {
            target.hash(state);
        }
    }
}

impl<Target> Default for RoleUpdateOperationScope<Target> {
    fn default() -> Self {
        let allowed = Default::default();
        Self::Restricted { allowed }
    }
}

/// Filters for workspace role update operation scope of the backend.
pub struct RoleUpdateOperationScopeFilters<'a, Target> {
    /// Equality workspace role update operation scope filter.
    pub eq: Option<Equal<&'a RoleUpdateOperationScope<Target>>>,
    /// Inequality workspace role update operation scope filter.
    pub ne: Option<NotEqual<&'a RoleUpdateOperationScope<Target>>>,
    /// In workspace role update operation scope filter.
    pub r#in: Option<In<&'a [RoleUpdateOperationScope<Target>]>>,
    /// Not in workspace role update operation scope filter.
    pub nin: Option<NotIn<&'a [RoleUpdateOperationScope<Target>]>>,
}

impl<'a, Target, Input> Filter<Input> for RoleUpdateOperationScopeFilters<'a, Target>
where
    Target: Eq + Hash,
    Input: Borrow<RoleUpdateOperationScope<Target>>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { eq, ne, r#in, nin } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}
