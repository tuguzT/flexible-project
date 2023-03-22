//! Filter model of the backend.

use std::borrow::Borrow;

pub use self::between::*;
pub use self::between_eq::*;
pub use self::eq::*;
pub use self::ge::*;
pub use self::gt::*;
pub use self::helpers::*;
pub use self::le::*;
pub use self::lt::*;
pub use self::ne::*;
pub use self::not::*;
pub use self::not_between::*;
pub use self::not_between_eq::*;
pub use self::not_in::*;
pub use self::r#in::*;
pub use self::regex::*;

mod between;
mod between_eq;
mod eq;
mod ge;
mod gt;
mod helpers;
mod r#in;
mod le;
mod lt;
mod ne;
mod not;
mod not_between;
mod not_between_eq;
mod not_in;
mod regex;

/// Defines behavior of filters of the backend.
pub trait Filter {
    /// Type of input to be checked by filter.
    type Input: ?Sized;

    /// Checks if input satisfies the filter.
    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>;
}

/// Input always satisfies the filter if filter is empty.
impl<T> Filter for Option<T>
where
    T: Filter,
{
    type Input = T::Input;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        match self {
            Some(filter) => filter.satisfies(input),
            None => true,
        }
    }
}
