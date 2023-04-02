use core::borrow::Borrow;

use super::Filter;

/// Not contains filter of the backend.
///
/// Checks if an input does not contain a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NotContains<T>(pub T);

impl<T> From<T> for NotContains<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input> Filter<Input> for NotContains<T>
where
    T: PartialEq,
    Input: IntoIterator,
    <Input as IntoIterator>::Item: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let mut input = input.into_iter();
        !input.any(|item| item.borrow() == value)
    }
}

#[cfg(test)]
mod tests {
    use core::{iter::once_with, ops::Not};

    use super::{Filter, NotContains};

    #[test]
    fn it_works() {
        let filter = NotContains(1);
        assert!(filter.satisfies(once_with(|| 1)).not());
        assert!(filter.satisfies(once_with(|| 2)));
        assert!(filter.satisfies([1, 2, 3, 4]).not());
        assert!(filter.satisfies([2, 3, 4, 5]));
    }
}
