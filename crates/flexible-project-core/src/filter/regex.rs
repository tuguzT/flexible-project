use std::borrow::Borrow;

use derive_more::From;
use fancy_regex::Regex as FancyRegex;

use super::Filter;

/// Regex filter of the backend.
///
/// Checks if input matches given regex pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct Regex<R>(pub R);

impl<S, Input> Filter<Input> for Regex<S>
where
    S: Borrow<str>,
    Input: Borrow<str>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(regex) = self;
        let regex = regex.borrow();
        let Ok(regex) = FancyRegex::new(regex) else {
            return false;
        };
        let input = input.borrow();
        regex.is_match(input).unwrap_or(false)
    }
}
