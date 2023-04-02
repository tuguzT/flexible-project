use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
};

use derive_more::From;

use super::Filter;

/// Contains filter of the backend.
///
/// Checks if an input contains a value.
#[derive(From)]
#[from(forward)]
pub struct Contains<'a, T>(pub Cow<'a, T>)
where
    T: PartialEq + ToOwned;

impl<T> Clone for Contains<'_, T>
where
    T: PartialEq + ToOwned,
{
    fn clone(&self) -> Self {
        let Self(value) = self;
        Self(value.clone())
    }
}

impl<T> Debug for Contains<'_, T>
where
    T: PartialEq + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(value) = self;
        f.debug_tuple("Contains").field(value).finish()
    }
}

impl<T, B, Input> Filter<Input> for Contains<'_, T>
where
    T: PartialEq + ToOwned,
    B: Borrow<T>,
    Input: IntoIterator<Item = B>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let mut input = input.into_iter();
        input.any(|item| item.borrow() == value.borrow())
    }
}
