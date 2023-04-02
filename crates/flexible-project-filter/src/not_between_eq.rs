use core::{borrow::Borrow, ops::RangeInclusive};

use super::Filter;

/// Not between equal filter of the backend.
///
/// Checks if input is less than or equal to (`<=`) lower bound
/// or bigger than or equal to (`>=`) higher bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, NotBetweenEqual};

    #[test]
    fn it_works() {
        let filter = NotBetweenEqual::from(1..=4);
        assert!(filter.satisfies(0));
        assert!(filter.satisfies(1));
        assert!(filter.satisfies(2).not());
        assert!(filter.satisfies(3).not());
        assert!(filter.satisfies(4));
        assert!(filter.satisfies(5));
    }
}
