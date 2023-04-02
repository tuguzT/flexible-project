use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
};

use derive_more::From;

use super::Filter;

/// Not contains filter of the backend.
///
/// Checks if an input does not contain a value.
#[derive(From)]
#[from(forward)]
pub struct NotContains<'a, T>(pub Cow<'a, T>)
where
    T: PartialEq + ToOwned;

impl<T> Clone for NotContains<'_, T>
where
    T: PartialEq + ToOwned,
{
    fn clone(&self) -> Self {
        let Self(value) = self;
        Self(value.clone())
    }
}

impl<T> Debug for NotContains<'_, T>
where
    T: PartialEq + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(value) = self;
        f.debug_tuple("NotContains").field(value).finish()
    }
}

impl<T, B, Input> Filter<Input> for NotContains<'_, T>
where
    T: PartialEq + Clone,
    B: Borrow<T>,
    Input: IntoIterator<Item = B>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let mut input = input.into_iter();
        !input.any(|item| item.borrow() == value.borrow())
    }
}
