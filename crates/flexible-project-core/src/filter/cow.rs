use std::borrow::Cow;

/// Extension trait used to turn shared reference into [`Cow::Borrowed`].
pub trait Borrowed: ToOwned {
    /// Converts shared reference into [`Cow::Borrowed`].
    fn borrowed(&self) -> Cow<'_, Self>;
}

impl<T> Borrowed for T
where
    T: ToOwned,
{
    fn borrowed(&self) -> Cow<'_, Self> {
        Cow::Borrowed(self)
    }
}

/// Extension trait used to turn owned data into [`Cow::Owned`].
pub trait Owned: ToOwned {
    /// Converts owned data into [`Cow::Owned`].
    fn owned(self) -> Cow<'static, Self>;
}

impl<T> Owned for T
where
    T: ToOwned<Owned = T>,
{
    fn owned(self) -> Cow<'static, Self> {
        Cow::Owned(self)
    }
}
