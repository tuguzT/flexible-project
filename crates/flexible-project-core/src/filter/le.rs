use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// Less equal filter of the backend.
///
/// Checks if input is less than or equal to (`<=`) the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct LessEqual<T>(pub T);

impl<T, Input> Filter<Input> for LessEqual<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input <= value
    }
}
