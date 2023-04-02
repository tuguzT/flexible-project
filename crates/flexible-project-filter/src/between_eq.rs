use core::{borrow::Borrow, ops::RangeInclusive};

use super::Filter;

/// Between equal filter of the backend.
///
/// Checks if input is bigger than or equal to (`>=`) lower bound
/// and less than or equal to (`<=`) higher bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{BetweenEqual, Filter};

    #[test]
    fn it_works() {
        let filter = BetweenEqual::from(1..=4);
        assert!(filter.satisfies(0).not());
        assert!(filter.satisfies(1));
        assert!(filter.satisfies(2));
        assert!(filter.satisfies(3));
        assert!(filter.satisfies(4));
        assert!(filter.satisfies(5).not());
    }
}
