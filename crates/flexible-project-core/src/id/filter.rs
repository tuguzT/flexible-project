use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    marker::PhantomData,
};

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
    pub eq: Option<Equal<Cow<'a, Id<Owner>>>>,
    /// Inequality identifier filter.
    pub ne: Option<NotEqual<Cow<'a, Id<Owner>>>>,
    /// In identifier filter.
    pub r#in: Option<In<Cow<'a, [Id<Owner>]>>>,
    /// Not in identifier filter.
    pub nin: Option<NotIn<Cow<'a, [Id<Owner>]>>>,
}

impl<'a, Owner: 'a> IdFilters<'a, Owner> {
    /// Sets the owner type for an identifier filters explicitly.
    pub fn with_owner<Other: 'a>(self) -> IdFilters<'a, Other> {
        let Self {
            owner: _,
            eq,
            ne,
            r#in,
            nin,
        } = self;
        IdFilters {
            owner: PhantomData,
            eq: eq.map(|Equal(id)| {
                let id = id.into_owned().with_owner();
                Equal(Cow::Owned(id))
            }),
            ne: ne.map(|NotEqual(id)| {
                let id = id.into_owned().with_owner();
                NotEqual(Cow::Owned(id))
            }),
            r#in: r#in.map(|In(ids)| {
                let ids = ids.iter().cloned().map(Id::with_owner).collect();
                In(Cow::Owned(ids))
            }),
            nin: nin.map(|NotIn(ids)| {
                let ids = ids.iter().cloned().map(Id::with_owner).collect();
                NotIn(Cow::Owned(ids))
            }),
        }
    }

    /// Erases the owner of an identifier filters explicitly, turning it into [`ErasedIdFilters`].
    pub fn erase(self) -> ErasedIdFilters<'a> {
        IdFilters::with_owner(self)
    }
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
        eq.satisfies(Cow::Borrowed(input))
            && ne.satisfies(Cow::Borrowed(input))
            && r#in.as_ref().map(In::as_deref).satisfies(input)
            && nin.as_ref().map(NotIn::as_deref).satisfies(input)
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
        let Self {
            owner,
            eq,
            ne,
            r#in,
            nin,
        } = self;
        Self {
            owner: *owner,
            eq: eq.clone(),
            ne: ne.clone(),
            r#in: r#in.clone(),
            nin: nin.clone(),
        }
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
