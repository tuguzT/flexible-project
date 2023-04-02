use core::borrow::Borrow;

use super::Filter;

/// Equality filter of the backend.
///
/// Checks if input is equal (`==`) to the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Equal<T>(pub T);

impl<T> From<T> for Equal<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input> Filter<Input> for Equal<T>
where
    T: PartialEq,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input == value
    }
}
