//! Identifier utilities for the Flexible Project server model.

use async_graphql::{InputObject, ID};
use fp_core::model::id::{Id, IdFilters as CoreIdFilters};

/// Identifier filters of the Flexible Project server.
#[derive(InputObject, Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdFilters {
    /// Equal identifier filter.
    eq: Option<ID>,
    /// Not equal identifier filter.
    ne: Option<ID>,
    /// In container identifier filter.
    #[graphql(name = "in")]
    r#in: Option<Vec<ID>>,
    /// Not in container identifier filter.
    nin: Option<Vec<ID>>,
}

impl<Owner> From<IdFilters> for CoreIdFilters<Owner>
where
    Owner: ?Sized,
{
    fn from(filters: IdFilters) -> Self {
        #[inline]
        fn convert_id<Owner>(id: ID) -> Id<Owner>
        where
            Owner: ?Sized,
        {
            String::from(id).into()
        }

        Self {
            eq: filters.eq.map(|id| convert_id(id).into()),
            ne: filters.ne.map(|id| convert_id(id).into()),
            r#in: filters
                .r#in
                .map(|ids| ids.into_iter().map(convert_id).collect::<Vec<_>>().into()),
            nin: filters
                .nin
                .map(|ids| ids.into_iter().map(convert_id).collect::<Vec<_>>().into()),
        }
    }
}
