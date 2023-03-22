use std::borrow::{Borrow, Cow};

use derive_more::From;

use super::Filter;

/// In filter of the backend.
///
/// Checks if a set of values contains an input.
#[derive(Debug, Clone, From)]
#[from(forward)]
pub struct In<'a, T>(pub Cow<'a, [T]>)
where
    T: PartialEq + Clone;

impl<T> Filter for In<'_, T>
where
    T: PartialEq + Clone,
{
    type Input = T;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self(slice) = self;
        let input = input.borrow();
        slice.contains(input)
    }
}
