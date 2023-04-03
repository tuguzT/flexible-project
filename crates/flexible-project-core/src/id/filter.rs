use std::{borrow::Borrow, marker::PhantomData};

use fp_filter::{Equal, Filter, In, NotEqual, NotIn};
use typed_builder::TypedBuilder;

use super::model::Id;

/// Filters for identifiers of the backend.
#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct IdFilters<'a, Owner: 'a> {
    /// Owner of the identifier.
    #[builder(setter(skip))]
    pub owner: PhantomData<fn() -> Owner>,
    /// Equality identifier filter.
    pub eq: Option<Equal<&'a Id<Owner>>>,
    /// Inequality identifier filter.
    pub ne: Option<NotEqual<&'a Id<Owner>>>,
    /// In identifier filter.
    pub r#in: Option<In<&'a [Id<Owner>]>>,
    /// Not in identifier filter.
    pub nin: Option<NotIn<&'a [Id<Owner>]>>,
}

impl<Owner, Input> Filter<Input> for IdFilters<'_, Owner>
where
    Input: Borrow<Id<Owner>>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self {
            owner: _,
            eq,
            ne,
            r#in,
            nin,
        } = self;
        let input = input.borrow();
        eq.satisfies(input) && ne.satisfies(input) && r#in.satisfies(input) && nin.satisfies(input)
    }
}

impl<Owner> Default for IdFilters<'_, Owner> {
    fn default() -> Self {
        Self {
            owner: Default::default(),
            eq: Default::default(),
            ne: Default::default(),
            r#in: Default::default(),
            nin: Default::default(),
        }
    }
}

impl<Owner> Clone for IdFilters<'_, Owner> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Owner> Copy for IdFilters<'_, Owner> {}
