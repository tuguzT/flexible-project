use core::borrow::Borrow;

use super::Filter;

/// Contains filter of the backend.
///
/// Checks if an input contains a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Contains<T>(pub T);

impl<T> From<T> for Contains<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input> Filter<Input> for Contains<T>
where
    T: PartialEq,
    Input: IntoIterator,
    <Input as IntoIterator>::Item: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let mut input = input.into_iter();
        input.any(|item| item.borrow() == value)
    }
}

#[cfg(test)]
mod tests {
    use core::{iter::once_with, ops::Not};

    use super::{Contains, Filter};

    #[test]
    fn it_works() {
        let filter = Contains(1);
        assert!(filter.satisfies(once_with(|| 1)));
        assert!(filter.satisfies(once_with(|| 2)).not());
        assert!(filter.satisfies([1, 2, 3, 4]));
        assert!(filter.satisfies([2, 3, 4, 5]).not());
    }
}
