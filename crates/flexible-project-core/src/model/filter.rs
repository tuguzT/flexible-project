//! Filter definitions and utilities for the Flexible Project system model.

use derive_more::From;

/// In container filter of the Flexible Project system.
///
/// Checks if the container contains the input.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct In<T>(pub Vec<T>)
where
    T: PartialEq;

/// Not in container filter of the Flexible Project system.
///
/// Checks if the container does not contain the input.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotIn<T>(pub Vec<T>)
where
    T: PartialEq;

/// Equality filter of the Flexible Project system.
///
/// Checks if input is equal (`==`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Equal<T>(pub T)
where
    T: PartialEq;

/// Inequality filter of the Flexible Project system.
///
/// Checks if input is not equal (`!=`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotEqual<T>(pub T)
where
    T: PartialEq;

/// Less than filter of the Flexible Project system.
///
/// Checks if input is less than (`<`) the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct LessThan<T>(pub T)
where
    T: PartialOrd;

/// Less than or equal filter of the Flexible Project system.
///
/// Checks if input is less than or equal (`<=`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct LessEqual<T>(pub T)
where
    T: PartialOrd;

/// Greater than filter of the Flexible Project system.
///
/// Checks if input is greater than (`>`) the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct GreaterThan<T>(pub T)
where
    T: PartialOrd;

/// Greater than or equal filter of the Flexible Project system.
///
/// Checks if input is greater than or equal (`>=`) to the value.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct GreaterEqual<T>(pub T)
where
    T: PartialOrd;

/// Between filter of the Flexible Project system.
///
/// Checks if input is located between the range bounds.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Between<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

/// Not between filter of the Flexible Project system.
///
/// Checks if input is not located between the range bounds.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotBetween<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

/// Between or equal filter of the Flexible Project system.
///
/// Checks if input is located between or equal to the range bounds.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct BetweenEqual<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

/// Not between or equal filter of the Flexible Project system.
///
/// Checks if input is not located between or equal to the range bounds.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct NotBetweenEqual<T>
where
    T: PartialOrd,
{
    /// Lower bound of the range.
    pub min: T,
    /// Higher bound of the range.
    pub max: T,
}

/// Contains filter of the Flexible Project system.
///
/// Checks if input contains the substring.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Contains(pub String);

/// Regex filter of the Flexible Project system.
///
/// Checks if input matches the provided regex.
#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord, From)]
pub struct Regex(pub String);
