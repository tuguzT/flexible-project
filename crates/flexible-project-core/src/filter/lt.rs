use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// Less than filter of the backend.
///
/// Checks if input is less than (`<`) inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct LessThan<T>(pub T);

impl<T, Input> Filter<Input> for LessThan<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input < value
    }
}
