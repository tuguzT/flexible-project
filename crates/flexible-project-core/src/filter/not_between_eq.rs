use std::{borrow::Borrow, ops::RangeInclusive};

use derive_more::From;

use super::Filter;

/// Not between equal filter of the backend.
///
/// Checks if input is less than or equal to (`<=`) lower bound
/// or bigger than or equal to (`>=`) higher bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct NotBetweenEqual<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<RangeInclusive<T>> for NotBetweenEqual<T> {
    fn from(range: RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        Self {
            min: start,
            max: end,
        }
    }
}

impl<T, Input> Filter<Input> for NotBetweenEqual<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { min, max } = self;
        let input = input.borrow();
        input <= min || max <= input
    }
}
