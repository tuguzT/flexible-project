use std::borrow::{Borrow, Cow};

use derive_more::From;

use super::Filter;

/// Not contains filter of the backend.
///
/// Checks if an input does not contain a value.
#[derive(Debug, Clone, From)]
#[from(forward)]
pub struct NotContains<'a, T>(pub Cow<'a, T>)
where
    T: PartialEq + Clone;

impl<'a, T> Filter for NotContains<'a, T>
where
    T: PartialEq + Clone,
{
    type Input = [T];

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self(value) = self;
        let input: &[_] = input.borrow();
        !input.contains(value)
    }
}
