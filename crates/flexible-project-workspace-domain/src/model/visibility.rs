use std::borrow::Borrow;

use derive_more::Display;
use fp_core::filter::{Equal, Filter, In, NotEqual, NotIn};
use typed_builder::TypedBuilder;

/// Visibility level of the workspace from outside of it.
#[derive(Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Visibility {
    /// Workspace is visible for any user of the system.
    #[default]
    Public,
    /// Workspace is only visible for members of this workspace.
    Private,
}

/// Filters for workspace visibility of the backend.
#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct VisibilityFilters<'a> {
    /// Equality workspace visibility filter.
    pub eq: Option<Equal<'a, Visibility>>,
    /// Inequality workspace visibility filter.
    pub ne: Option<NotEqual<'a, Visibility>>,
    /// In workspace visibility filter.
    pub r#in: Option<In<'a, Visibility>>,
    /// Not in workspace visibility filter.
    pub nin: Option<NotIn<'a, Visibility>>,
}

impl<Input> Filter<Input> for VisibilityFilters<'_>
where
    Input: Borrow<Visibility>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { eq, ne, r#in, nin } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}
