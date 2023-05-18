use core::borrow::Borrow;

use super::{CowSlice, Filter};

/// Not in filter of the backend.
///
/// Checks if a set of values does not contain an input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NotIn<T>(pub T);

impl<T> From<T> for NotIn<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input, Item> Filter<Input> for NotIn<T>
where
    T: IntoIterator<Item = Item> + Clone,
    Input: Borrow<Item>,
    Item: PartialEq,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let mut iter = value.clone().into_iter();
        let input = input.borrow();
        !iter.any(|item| &item == input)
    }
}

impl<'a, T, Input> Filter<Input> for NotIn<CowSlice<'a, T>>
where
    T: PartialEq + Clone + 'a,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(CowSlice(cow)) = self;
        let slice: &[_] = cow.borrow();
        let input = input.borrow();
        !slice.contains(input)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, NotIn};

    #[test]
    fn it_works() {
        let filter = NotIn([1, 3, 5]);
        assert!(filter.satisfies(0));
        assert!(filter.satisfies(1).not());
        assert!(filter.satisfies(2));
        assert!(filter.satisfies(3).not());
        assert!(filter.satisfies(4));
        assert!(filter.satisfies(5).not());
        assert!(filter.satisfies(6));
    }
}
