use core::{borrow::Borrow, ops::Range};

use super::Filter;

/// Between filter of the backend.
///
/// Checks if input is bigger than (`>`) lower bound
/// and less than (`<`) higher bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Between<T> {
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> From<Range<T>> for Between<T> {
    fn from(range: Range<T>) -> Self {
        let Range { start, end } = range;
        Self {
            min: start,
            max: end,
        }
    }
}

impl<T, Input> Filter<Input> for Between<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self { min, max } = self;
        let input = input.borrow();
        min < input && input < max
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Between, Filter};

    #[test]
    fn it_works() {
        let filter = Between::from(1..4);
        assert!(filter.satisfies(0).not());
        assert!(filter.satisfies(1).not());
        assert!(filter.satisfies(2));
        assert!(filter.satisfies(3));
        assert!(filter.satisfies(4).not());
        assert!(filter.satisfies(5).not());
    }
}
