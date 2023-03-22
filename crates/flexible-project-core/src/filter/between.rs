use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    ops::Range,
};

use derive_more::From;

use super::Filter;

/// Between filter of the backend.
///
/// Checks if input is bigger than (`>`) lower bound
/// and less than (`<`) higher bound.
#[derive(From)]
#[from(forward)]
pub struct Between<'a, T>
where
    T: PartialOrd + ToOwned,
{
    /// Lower bound of the range.
    pub min: Cow<'a, T>,
    /// Higher bound of the range.
    pub max: Cow<'a, T>,
}

impl<T> Clone for Between<'_, T>
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

impl<'a, T> Debug for Between<'a, T>
where
    T: PartialOrd + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { min, max } = self;
        f.debug_struct("Between")
            .field("min", min)
            .field("max", max)
            .finish()
    }
}

impl<T> Filter for Between<'_, T>
where
    T: PartialOrd + ToOwned,
{
    type Input = T;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self { min, max } = self;
        let min: &T = min.borrow();
        let max: &T = max.borrow();
        let input = input.borrow();
        min < input && input < max
    }
}

impl<'a, T> From<Range<Cow<'a, T>>> for Between<'a, T>
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
