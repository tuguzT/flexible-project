//! Filter model of the backend.

use auto_impl::auto_impl;

pub use self::{
    between::Between,
    between_eq::BetweenEqual,
    contains::Contains,
    cow::{Borrowed, Owned},
    eq::Equal,
    ge::GreaterEqual,
    gt::GreaterThan,
    le::LessEqual,
    lt::LessThan,
    ne::NotEqual,
    not::Not,
    not_between::NotBetween,
    not_between_eq::NotBetweenEqual,
    not_contains::NotContains,
    not_in::NotIn,
    r#in::In,
    regex::Regex,
};

mod between;
mod between_eq;
mod contains;
mod cow;
mod eq;
mod ge;
mod gt;
mod r#in;
mod le;
mod lt;
mod ne;
mod not;
mod not_between;
mod not_between_eq;
mod not_contains;
mod not_in;
mod regex;

/// Defines behavior of filters of the backend.
#[auto_impl(&, Box, Rc, Arc)]
pub trait Filter<Input> {
    /// Checks if input satisfies the filter.
    fn satisfies(&self, input: Input) -> bool;
}

/// Input always satisfies the filter if filter is empty.
impl<F, Input> Filter<Input> for Option<F>
where
    F: Filter<Input>,
{
    fn satisfies(&self, input: Input) -> bool {
        match self {
            Some(filter) => filter.satisfies(input),
            None => true,
        }
    }
}
