use core::borrow::Borrow;

use super::Filter;

/// Inequality filter of the backend.
///
/// Checks if input is not equal (`!=`) to the inner value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NotEqual<T>(pub T);

impl<T> From<T> for NotEqual<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, Input> Filter<Input> for NotEqual<T>
where
    T: PartialEq,
    Input: Borrow<T>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(value) = self;
        let input = input.borrow();
        input != value
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, NotEqual};

    #[test]
    fn it_works() {
        let filter = NotEqual(1);
        assert!(filter.satisfies(1).not());
        assert!(filter.satisfies(0));
    }
}
