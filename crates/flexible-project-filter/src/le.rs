use core::borrow::Borrow;

use super::Filter;

/// Less equal filter of the backend.
///
/// Checks if input is less than or equal to (`<=`) the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LessEqual<T>(pub T);

impl<T> From<T> for LessEqual<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input> Filter<Input> for LessEqual<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input <= value
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, LessEqual};

    #[test]
    fn it_works() {
        let filter = LessEqual(1);
        assert!(filter.satisfies(0));
        assert!(filter.satisfies(1));
        assert!(filter.satisfies(2).not());
    }
}
