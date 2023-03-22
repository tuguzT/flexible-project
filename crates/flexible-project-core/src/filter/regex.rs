use std::borrow::{Borrow, Cow};

use derive_more::From;

use super::Filter;

/// Regex filter of the backend.
///
/// Checks if input matches given regex pattern.
#[derive(Debug, Clone, From)]
pub struct Regex<'a>(pub Cow<'a, str>);

impl Filter for Regex<'_> {
    type Input = str;

    fn satisfies<B>(&self, input: B) -> bool
    where
        B: Borrow<Self::Input>,
    {
        let Self(regex) = self;
        let input = input.borrow();
        let Ok(regex) = fancy_regex::Regex::new(regex) else {
            return false;
        };
        regex.is_match(input).unwrap_or(false)
    }
}
