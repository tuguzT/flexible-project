use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// Greater equal filter of the backend.
///
/// Checks if input is greater than or equal to (`>=`) the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct GreaterEqual<T>(pub T);

impl<T, Input> Filter<Input> for GreaterEqual<T>
where
    T: PartialOrd,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input >= value
    }
}
