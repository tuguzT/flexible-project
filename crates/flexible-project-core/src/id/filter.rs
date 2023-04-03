use std::{borrow::Borrow, fmt::Debug, marker::PhantomData};

use fp_filter::{Equal, Filter, In, NotEqual, NotIn};
use typed_builder::TypedBuilder;

use super::model::{ErasedOwner, Id};

/// Filters for erased identifier of the backend.
pub type ErasedIdFilters<'a> = IdFilters<'a, ErasedOwner>;

/// Filters for identifier of the backend.
#[derive(TypedBuilder)]
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

impl<Owner> Debug for IdFilters<'_, Owner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdFilters")
            .field("owner", &self.owner)
            .field("eq", &self.eq)
            .field("ne", &self.ne)
            .field("r#in", &self.r#in)
            .field("nin", &self.nin)
            .finish()
    }
}

impl<Owner> Clone for IdFilters<'_, Owner> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Owner> Copy for IdFilters<'_, Owner> {}

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
