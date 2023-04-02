use std::borrow::{Borrow, Cow};

use derive_more::From;

use super::Filter;

/// Not in filter of the backend.
///
/// Checks if a set of values does not contain an input.
#[derive(Debug, Clone, From)]
#[from(forward)]
pub struct NotIn<'a, T>(pub Cow<'a, [T]>)
where
    T: PartialEq + Clone;

impl<T, Input> Filter<Input> for NotIn<'_, T>
where
    T: PartialEq + Clone,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(slice) = self;
        let input = input.borrow();
        !slice.contains(input)
    }
}
