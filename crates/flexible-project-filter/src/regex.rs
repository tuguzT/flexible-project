use core::borrow::Borrow;

use fancy_regex::Regex as FancyRegex;

use super::Filter;

/// Regex filter of the backend.
///
/// Checks if input matches given regex pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Regex<R>(pub R);

impl<R> From<R> for Regex<R> {
    fn from(value: R) -> Self {
        Self(value)
    }
}

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

#[cfg(test)]
mod tests {
    use core::ops::Not;

    use super::{Filter, Regex};

    #[test]
    fn it_works() {
        let filter = Regex(r#"^((?=\S*?[A-Z])(?=\S*?[a-z])(?=\S*?[0-9]).{6,})\S$"#);
        assert!(filter.satisfies("Catcat1"));
        assert!(filter.satisfies("smol").not());
    }
}
