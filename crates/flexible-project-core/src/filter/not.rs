use derive_more::From;

use super::Filter;

/// Negation filter of the backend.
///
/// Checks if input does not satisfy the inner filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct Not<F>(pub F);

impl<F, Input> Filter<Input> for Not<F>
where
    F: Filter<Input>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(filter) = self;
        !filter.satisfies(input)
    }
}
