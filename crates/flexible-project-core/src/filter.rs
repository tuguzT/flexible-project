//! Filter model of the backend.

use std::ops::{Range, RangeInclusive};

use derive_more::From;

/// Defines behavior of filters of the backend.
pub trait Filter {
    /// Type of input to be checked by filter.
    type Input<'a>
    where
        Self: 'a;

    /// Checks if input satisfies the filter.
    fn satisfies(&self, input: Self::Input<'_>) -> bool;
}

impl<T> Filter for Option<T>
where
    T: Filter,
{
    type Input<'a> = T::Input<'a>
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        match self {
            Some(filter) => filter.satisfies(input),
            None => true,
        }
    }
}

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
    type Input<'a> = F::Input<'a>
    where
        F: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(filter) = self;
        !filter.satisfies(input)
    }
}

/// Equality filter of the backend.
///
/// Checks if input is equal (`==`) to the inner value.
#[derive(Debug, Clone, Copy, From)]
pub struct Equal<T>(pub T)
where
    T: PartialEq;

impl<T> Filter for Equal<T>
where
    T: PartialEq,
{
    type Input<'a> = &'a T
    where
        T: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(value) = self;
        value == input
    }
}

/// Inequality filter of the backend.
///
/// Checks if input is not equal (`!=`) to the inner value.
#[derive(Debug, Clone, Copy, From)]
pub struct NotEqual<T>(pub T)
where
    T: PartialEq;

impl<T> Filter for NotEqual<T>
where
    T: PartialEq,
{
    type Input<'a> = &'a T
    where
        T: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(value) = self;
        value != input
    }
}

/// Less than filter of the backend.
///
/// Checks if input is less than (`<`) inner value.
#[derive(Debug, Clone, Copy, From)]
pub struct LessThan<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for LessThan<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(value) = self;
        input < value
    }
}

/// Less equal filter of the backend.
///
/// Checks if input is less than or equal to (`<=`) the inner value.
#[derive(Debug, Clone, Copy, From)]
pub struct LessEqual<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for LessEqual<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(value) = self;
        input <= value
    }
}

/// Greater than filter of the backend.
///
/// Checks if input is greater than (`>`) inner value.
#[derive(Debug, Clone, Copy, From)]
pub struct GreaterThan<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for GreaterThan<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(value) = self;
        input > value
    }
}

/// Greater equal filter of the backend.
///
/// Checks if input is greater than or equal to (`>=`) the inner value.
#[derive(Debug, Clone, Copy, From)]
pub struct GreaterEqual<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for GreaterEqual<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
        where
            Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(value) = self;
        input >= value
    }
}

/// Between filter of the backend.
///
/// Checks if input is bigger than (`>`) lower bound
/// and less than (`<`) higher bound.
#[derive(Debug, Clone, Copy, From)]
pub struct Between<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> Filter for Between<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        T: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self { min, max } = self;
        min < input && input < max
    }
}

impl<T> From<Range<T>> for Between<T>
where
    T: PartialOrd,
{
    fn from(range: Range<T>) -> Self {
        let Range { start, end } = range;
        Self {
            min: start,
            max: end,
        }
    }
}

/// Not between filter of the backend.
///
/// Checks if input is less than (`<`) lower bound
/// or bigger than (`>`) higher bound.
#[derive(Debug, Clone, Copy, From)]
pub struct NotBetween<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> Filter for NotBetween<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self { min, max } = self;
        input < min || max < input
    }
}

impl<T> From<Range<T>> for NotBetween<T>
where
    T: PartialOrd,
{
    fn from(range: Range<T>) -> Self {
        let Range { start, end } = range;
        Self {
            min: start,
            max: end,
        }
    }
}

/// Between equal filter of the backend.
///
/// Checks if input is bigger than or equal to (`>=`) lower bound
/// and less than or equal to (`<=`) higher bound.
#[derive(Debug, Clone, Copy, From)]
pub struct BetweenEqual<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> Filter for BetweenEqual<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        T: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self { min, max } = self;
        min <= input && input <= max
    }
}

impl<T> From<RangeInclusive<T>> for BetweenEqual<T>
where
    T: PartialOrd,
{
    fn from(range: RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        Self {
            min: start,
            max: end,
        }
    }
}

/// Not between equal filter of the backend.
///
/// Checks if input is less than or equal to (`<=`) lower bound
/// or bigger than or equal to (`>=`) higher bound.
#[derive(Debug, Clone, Copy, From)]
pub struct NotBetweenEqual<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

impl<T> Filter for NotBetweenEqual<T>
where
    T: PartialOrd,
{
    type Input<'a> = &'a T
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self { min, max } = self;
        input <= min || max <= input
    }
}

impl<T> From<RangeInclusive<T>> for NotBetweenEqual<T>
where
    T: PartialOrd,
{
    fn from(range: RangeInclusive<T>) -> Self {
        let (start, end) = range.into_inner();
        Self {
            min: start,
            max: end,
        }
    }
}

/// In filter of the backend.
///
/// Checks if a set of values contains an input.
#[derive(Debug, Clone, From)]
pub struct In<T>(pub Vec<T>)
where
    T: PartialEq;

impl<T> Filter for In<T>
where
    T: PartialEq,
{
    type Input<'a> = &'a T
    where
        T: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(vec) = self;
        vec.contains(input)
    }
}

/// Not in filter of the backend.
///
/// Checks if a set of values does not contain an input.
#[derive(Debug, Clone, From)]
pub struct NotIn<T>(pub Vec<T>)
where
    T: PartialEq;

impl<T> Filter for NotIn<T>
where
    T: PartialEq,
{
    type Input<'a> = &'a T
    where
        T: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(vec) = self;
        !vec.contains(input)
    }
}

/// Regex filter of the backend.
///
/// Checks if input matches given regex pattern.
#[derive(Debug, Clone, From)]
pub struct Regex(pub String);

impl Filter for Regex {
    type Input<'a> = &'a str
    where
        Self: 'a;

    fn satisfies(&self, input: Self::Input<'_>) -> bool {
        let Self(regex) = self;
        let Ok(regex) = fancy_regex::Regex::new(regex) else {
            return false;
        };
        regex.is_match(input).unwrap_or(false)
    }
}
