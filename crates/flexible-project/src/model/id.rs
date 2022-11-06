//! Identifier utilities for the Flexible Project server model.

use async_graphql::{InputObject, ID};
use fp_core::model::filter::Equal;
use fp_core::model::id::IdFilters as CoreIdFilters;

/// Identifier filters of the Flexible Project server.
#[derive(InputObject, Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdFilters {
    // #[graphql(name = "in")]
    // r#in: Option<Vec<ID>>,
    eq: Option<ID>,
}

impl<Owner> From<IdFilters> for CoreIdFilters<Owner>
// TODO: linked with IdFilters from domain layer
// where
//     Owner: ?Sized,
{
    fn from(filters: IdFilters) -> Self {
        Self {
            eq: filters.eq.map(|id| Equal(String::from(id).into())),
            ne: Default::default(),
            r#in: Default::default(),
            nin: Default::default(),
        }
    }
}
