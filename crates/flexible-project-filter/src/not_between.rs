use core::{borrow::Borrow, ops::Range};

use super::Filter;

/// Not between filter of the backend.
///
/// Checks if input is less than (`<`) lower bound
/// or bigger than (`>`) higher bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NotBetween<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<Range<T>> for NotBetween<T> {
    fn from(range: Range<T>) -> Self {
        let Range { start, end } = range;
        Self {
            min: start,
            max: end,
        }
    }
}

impl<T, Input> Filter<Input> for NotBetween<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { min, max } = self;
        let input = input.borrow();
        input < min || max < input
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, NotBetween};

    #[test]
    fn it_works() {
        let filter = NotBetween::from(1..4);
        assert!(filter.satisfies(0));
        assert!(filter.satisfies(1).not());
        assert!(filter.satisfies(2).not());
        assert!(filter.satisfies(3).not());
        assert!(filter.satisfies(4).not());
        assert!(filter.satisfies(5));
    }
}
