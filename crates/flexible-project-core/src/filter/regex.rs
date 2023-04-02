use std::borrow::{Borrow, Cow};

use derive_more::From;
use fancy_regex::Regex as FancyRegex;

use super::Filter;

/// Regex filter of the backend.
///
/// Checks if input matches given regex pattern.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
#[from(forward)]
pub struct Regex<'a>(pub Cow<'a, str>);

impl<Input> Filter<Input> for Regex<'_>
where
    Input: Borrow<str>,
{
    fn satisfies(&self, input: Input) -> bool {
        let Self(regex) = self;
        let input = input.borrow();
        let Ok(regex) = FancyRegex::new(regex) else {
            return false;
        };
        regex.is_match(input).unwrap_or(false)
    }
}
