use std::{borrow::Borrow, ops::RangeInclusive};

use derive_more::From;

use super::Filter;

/// Between equal filter of the backend.
///
/// Checks if input is bigger than or equal to (`>=`) lower bound
/// and less than or equal to (`<=`) higher bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct BetweenEqual<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<RangeInclusive<T>> for BetweenEqual<T> {
    fn from(range: RangeInclusive<T>) -> Self {
        let (min, max) = range.into_inner();
        Self { min, max }
    }
}

impl<T, Input> Filter<Input> for BetweenEqual<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { min, max } = self;
        let input = input.borrow();
        min <= input && input <= max
    }
}
