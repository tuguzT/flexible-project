use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// Inequality filter of the backend.
///
/// Checks if input is not equal (`!=`) to the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct NotEqual<T>(pub T);

impl<T, Input> Filter<Input> for NotEqual<T>
where
    T: PartialEq,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input != value
    }
}
