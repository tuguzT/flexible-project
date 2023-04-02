use core::borrow::Borrow;

use super::Filter;

/// Greater than filter of the backend.
///
/// Checks if input is greater than (`>`) inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GreaterThan<T>(pub T);

impl<T> From<T> for GreaterThan<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input> Filter<Input> for GreaterThan<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input > value
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, GreaterThan};

    #[test]
    fn it_works() {
        let filter = GreaterThan(1);
        assert!(filter.satisfies(2));
        assert!(filter.satisfies(1).not());
        assert!(filter.satisfies(0).not());
    }
}
