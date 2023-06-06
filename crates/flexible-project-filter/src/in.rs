use core::{borrow::Borrow, ops::Deref};

use super::Filter;

/// In filter of the backend.
///
/// Checks if a set of values contains an input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct In<T>(pub T);

impl<T> In<T>
where
    T: Deref,
{
    /// Converts from `In<T>` (or `&In<T>`) to `In<&T::Target>`.
    pub fn as_deref(&self) -> In<&T::Target> {
        let In(values) = self;
        In(values)
    }
}

impl<T> From<T> for In<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input, Item> Filter<Input> for In<T>
where
    T: IntoIterator<Item = Item> + Clone,
    Input: Borrow<Item>,
    Item: PartialEq,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let mut iter = value.clone().into_iter();
        let input = input.borrow();
        iter.any(|item| &item == input)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, In};

    #[test]
    fn it_works() {
        let filter = In([1, 3, 5]);
        assert!(filter.satisfies(0).not());
        assert!(filter.satisfies(1));
        assert!(filter.satisfies(2).not());
        assert!(filter.satisfies(3));
        assert!(filter.satisfies(4).not());
        assert!(filter.satisfies(5));
        assert!(filter.satisfies(6).not());
    }
}
