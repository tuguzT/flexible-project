use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
};

use derive_more::From;

use super::Filter;

/// Greater equal filter of the backend.
///
/// Checks if input is greater than or equal to (`>=`) the inner value.
#[derive(From)]
#[from(forward)]
pub struct GreaterEqual<'a, T>(pub Cow<'a, T>)
where
    T: PartialOrd + ToOwned;

impl<T> Clone for GreaterEqual<'_, T>
where
    T: PartialOrd + ToOwned,
{
    fn clone(&self) -> Self {
        let Self(value) = self;
        Self(value.clone())
    }
}

impl<T> Debug for GreaterEqual<'_, T>
where
    T: PartialOrd + ToOwned + Debug,
    <T as ToOwned>::Owned: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(value) = self;
        f.debug_tuple("GreaterEqual").field(value).finish()
    }
}

impl<T> Filter for GreaterEqual<'_, T>
where
    T: PartialOrd + ToOwned,
{
    type Input = T;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self(value) = self;
        input.borrow() >= value.borrow()
    }
}
