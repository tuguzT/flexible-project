use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
};

use derive_more::From;

use super::Filter;

/// Equality filter of the backend.
///
/// Checks if input is equal (`==`) to the inner value.
#[derive(From)]
#[from(forward)]
pub struct Equal<'a, T>(pub Cow<'a, T>)
where
    T: PartialEq + ToOwned;

impl<T> Clone for Equal<'_, T>
where
    T: PartialEq + ToOwned,
{
    fn clone(&self) -> Self {
        let Self(value) = self;
        Self(value.clone())
    }
}

impl<T> Debug for Equal<'_, T>
where
    T: PartialEq + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(value) = self;
        f.debug_tuple("Equal").field(value).finish()
    }
}

impl<T> Filter for Equal<'_, T>
where
    T: PartialEq + ToOwned,
{
    type Input = T;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self(value) = self;
        input.borrow() == value.borrow()
    }
}
