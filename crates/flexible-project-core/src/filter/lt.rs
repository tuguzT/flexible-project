use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
};

use derive_more::From;

use super::Filter;

/// Less than filter of the backend.
///
/// Checks if input is less than (`<`) inner value.
#[derive(From)]
#[from(forward)]
pub struct LessThan<'a, T>(pub Cow<'a, T>)
where
    T: PartialOrd + ToOwned;

impl<T> Clone for LessThan<'_, T>
where
    T: PartialOrd + ToOwned,
{
    fn clone(&self) -> Self {
        let Self(value) = self;
        Self(value.clone())
    }
}

impl<T> Debug for LessThan<'_, T>
where
    T: PartialOrd + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(value) = self;
        f.debug_tuple("LessThan").field(value).finish()
    }
}

impl<T, Input> Filter<Input> for LessThan<'_, T>
where
    T: PartialOrd + ToOwned,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        input.borrow() < value.borrow()
    }
}
