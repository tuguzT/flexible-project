use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// In filter of the backend.
///
/// Checks if a set of values contains an input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct In<T>(pub T)
where
    T: IntoIterator;

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
