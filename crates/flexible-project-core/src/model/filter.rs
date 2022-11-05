//! Filter utilities for the Flexible Project system model.

use std::convert::Infallible;
use std::hash::{Hash, Hasher};
use std::ops::Not as _;

use derive_more::{Display, Error, From};

/// Type of filter for the Flexible Project system.
pub trait Filter {
    /// Input type which will be checked by filter type.
    type Input: ?Sized;

    /// Error type which will be returned if any error occurs.
    type Error;

    /// Checks if the input object satisfies this filter.
    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error>;
}

/// Not filter of the Flexible Project system.
///
/// Allows to inverse check result of the inner filter.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Not<F>(pub F)
where
    F: Filter;

impl<F> Filter for Not<F>
where
    F: Filter,
{
    type Input = F::Input;

    type Error = F::Error;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = self.0.check(input)?.not();
        Ok(result)
    }
}

/// In container filter of the Flexible Project system.
///
/// Checks if the container contains the input.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct In<T>(pub Vec<T>)
where
    T: PartialEq;

impl<T> Filter for In<T>
where
    T: PartialEq,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = self.0.contains(input);
        Ok(result)
    }
}

/// Not in container filter of the Flexible Project system.
///
/// Checks if the container does not contain the input.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotIn<T>(pub Vec<T>)
where
    T: PartialEq;

impl<T> Filter for NotIn<T>
where
    T: PartialEq,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = self.0.contains(input).not();
        Ok(result)
    }
}

/// Equality filter of the Flexible Project system.
///
/// Checks if input is equal (`==`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Equal<T>(pub T)
where
    T: PartialEq;

impl<T> Filter for Equal<T>
where
    T: PartialEq,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        Ok(input == &self.0)
    }
}

/// Inequality filter of the Flexible Project system.
///
/// Checks if input is not equal (`!=`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotEqual<T>(pub T)
where
    T: PartialEq;

impl<T> Filter for NotEqual<T>
where
    T: PartialEq,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        Ok(input != &self.0)
    }
}

/// Less than filter of the Flexible Project system.
///
/// Checks if input is less than (`<`) the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct LessThan<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for LessThan<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        Ok(input < &self.0)
    }
}

/// Less than or equal filter of the Flexible Project system.
///
/// Checks if input is less than or equal (`<=`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct LessEqual<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for LessEqual<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        Ok(input <= &self.0)
    }
}

/// Greater than filter of the Flexible Project system.
///
/// Checks if input is greater than (`>`) the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct GreaterThan<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for GreaterThan<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        Ok(input > &self.0)
    }
}

/// Greater than or equal filter of the Flexible Project system.
///
/// Checks if input is greater than or equal (`>=`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct GreaterEqual<T>(pub T)
where
    T: PartialOrd;

impl<T> Filter for GreaterEqual<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        Ok(input >= &self.0)
    }
}

/// Between filter of the Flexible Project system.
///
/// Checks if input is located between left and right values.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Between<T>
where
    T: PartialOrd,
{
    left: T,
    right: T,
}

impl<T> Filter for Between<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = (&self.left < input) && (input < &self.right);
        Ok(result)
    }
}

/// Not between filter of the Flexible Project system.
///
/// Checks if input is not located between left and right values.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotBetween<T>
where
    T: PartialOrd,
{
    left: T,
    right: T,
}

impl<T> Filter for NotBetween<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = (input <= &self.left) || (&self.right <= input);
        Ok(result)
    }
}

/// Between or equal filter of the Flexible Project system.
///
/// Checks if input is located between or equal to the left and right values.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct BetweenEqual<T>
where
    T: PartialOrd,
{
    left: T,
    right: T,
}

impl<T> Filter for BetweenEqual<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = (&self.left <= input) && (input <= &self.right);
        Ok(result)
    }
}

/// Not between or equal filter of the Flexible Project system.
///
/// Checks if input is not located between or equal to the left and right values.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotBetweenEqual<T>
where
    T: PartialOrd,
{
    left: T,
    right: T,
}

impl<T> Filter for NotBetweenEqual<T>
where
    T: PartialOrd,
{
    type Input = T;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = (input < &self.left) || (&self.right < input);
        Ok(result)
    }
}

/// Contains filter of the Flexible Project system.
///
/// Checks if input contains the substring.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Contains(pub String);

impl Filter for Contains {
    type Input = str;

    type Error = Infallible;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = input.contains(&self.0);
        Ok(result)
    }
}

/// Regex filter error type.
#[derive(Debug, Display, Error, From)]
pub struct RegexError(fancy_regex::Error);

/// Regex filter of the Flexible Project system.
///
/// Checks if input matches the provided regex.
#[derive(Debug, Clone)]
pub struct Regex {
    regex: fancy_regex::Regex,
}

impl Regex {
    /// Creates new regex filter from the input string slice.
    pub fn new(regex: &str) -> Result<Self, RegexError> {
        let regex = fancy_regex::Regex::new(regex)?;
        Ok(Self { regex })
    }

    /// Returns the original string slice of this regex filter.
    pub fn as_str(&self) -> &str {
        self.regex.as_str()
    }
}

impl Filter for Regex {
    type Input = str;

    type Error = RegexError;

    fn check(&self, input: &Self::Input) -> Result<bool, Self::Error> {
        let result = self.regex.is_match(input)?;
        Ok(result)
    }
}

impl Hash for Regex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Regex {}

impl PartialOrd for Regex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Regex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}
