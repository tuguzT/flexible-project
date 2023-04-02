use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    ops::Range,
};

use derive_more::From;

use super::Filter;

/// Not between filter of the backend.
///
/// Checks if input is less than (`<`) lower bound
/// or bigger than (`>`) higher bound.
#[derive(From)]
#[from(forward)]
pub struct NotBetween<'a, T>
where
    T: PartialOrd + ToOwned,
{
    /// Lower bound of the range.
    pub min: Cow<'a, T>,
    /// Higher bound of the range.
    pub max: Cow<'a, T>,
}

impl<T> Clone for NotBetween<'_, T>
where
    T: PartialOrd + ToOwned,
{
    fn clone(&self) -> Self {
        let Self { min, max } = self;
        Self {
            min: min.clone(),
            max: max.clone(),
        }
    }
}

impl<'a, T> Debug for NotBetween<'a, T>
where
    T: PartialOrd + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { min, max } = self;
        f.debug_struct("NotBetween")
            .field("min", min)
            .field("max", max)
            .finish()
    }
}

impl<T, Input> Filter<Input> for NotBetween<'_, T>
where
    T: PartialOrd + ToOwned,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { min, max } = self;
        let min: &T = min.borrow();
        let max: &T = max.borrow();
        let input = input.borrow();
        input < min || max < input
    }
}

impl<'a, T> From<Range<Cow<'a, T>>> for NotBetween<'a, T>
where
    T: PartialOrd + ToOwned,
{
    fn from(range: Range<Cow<'a, T>>) -> Self {
        let Range { start, end } = range;
        Self {
            min: start,
            max: end,
        }
    }
}
