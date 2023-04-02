use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// Not contains filter of the backend.
///
/// Checks if an input does not contain a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct NotContains<T>(pub T);

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
