use std::borrow::Borrow;

use derive_more::From;

use super::Filter;

/// Negation filter of the backend.
///
/// Checks if input does not satisfy the inner filter.
#[derive(Debug, Clone, Copy, From)]
pub struct Not<F>(pub F)
where
    F: Filter;

impl<F> Filter for Not<F>
where
    F: Filter,
{
    type Input = F::Input;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self(filter) = self;
        !filter.satisfies(input)
    }
}
