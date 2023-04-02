use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    ops::RangeInclusive,
};

use derive_more::From;

use super::Filter;

/// Between equal filter of the backend.
///
/// Checks if input is bigger than or equal to (`>=`) lower bound
/// and less than or equal to (`<=`) higher bound.
#[derive(From)]
#[from(forward)]
pub struct BetweenEqual<'a, T>
where
    T: PartialOrd + ToOwned,
{
    /// Lower bound of the range.
    pub min: Cow<'a, T>,
    /// Higher bound of the range.
    pub max: Cow<'a, T>,
}

impl<T> Clone for BetweenEqual<'_, T>
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

impl<'a, T> Debug for BetweenEqual<'a, T>
where
    T: PartialOrd + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { min, max } = self;
        f.debug_struct("BetweenEqual")
            .field("min", min)
            .field("max", max)
            .finish()
    }
}

impl<T, Input> Filter<Input> for BetweenEqual<'_, T>
where
    T: PartialOrd + ToOwned,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { min, max } = self;
        let min: &T = min.borrow();
        let max: &T = max.borrow();
        let input = input.borrow();
        min <= input && input <= max
    }
}

impl<'a, T> From<RangeInclusive<Cow<'a, T>>> for BetweenEqual<'a, T>
where
    T: PartialOrd + ToOwned,
{
    fn from(range: RangeInclusive<Cow<'a, T>>) -> Self {
        let (start, end) = range.into_inner();
        Self {
            min: start,
            max: end,
        }
    }
}
