use std::borrow::Cow;

/// Special wrapper around [`Cow`]
/// for [`In`](crate::In) and [`NotIn`](crate::NotIn) filters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CowSlice<'a, T>(pub Cow<'a, [T]>)
where
    T: Clone + 'a;

impl<'a, T, F> From<F> for CowSlice<'a, T>
where
    T: Clone + 'a,
    Cow<'a, [T]>: From<F>,
{
    fn from(value: F) -> Self {
        Self(value.into())
    }
}
