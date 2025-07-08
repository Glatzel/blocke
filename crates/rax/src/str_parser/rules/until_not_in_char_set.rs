use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule that extracts a prefix from the input string up to (but not including)
/// the first character that is NOT in the provided character set filter.
/// Returns a tuple of (prefix, rest) where `prefix` contains all consecutive
/// characters from the start of the input that are in the set, and `rest` is
/// the remainder of the string starting from the first character not in the
/// set. If all characters are in the set, returns (None, input).
pub struct UntilNotInCharSet<'a, const N: usize>(pub &'a CharSetFilter<N>);

impl<'a, const N: usize> IRule for UntilNotInCharSet<'a, N> {
    fn name(&self) -> &str { "UntilNotInCharSet" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilNotInCharSet<'a, N> {
    type Output = &'a str;

    /// Applies the rule to the input string, returning the prefix of characters
    /// in the set and the rest of the string starting from the first character
    /// not in the set. If all characters are in the set, returns (None, input).
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Iterate over each character and its byte index in the input string
        for (i, c) in input.char_indices() {
            // If the character is NOT in the set, split here
            if !self.0.filter(&c) {
                let prefix = &input[..i]; // All chars up to (but not including) this one
                let rest = &input[i..]; // The rest of the string starting from this char
                clerk::debug!(
                    "UntilOneOfCharSet matched: prefix='{}', rest='{}, i={}, c='{}'",
                    prefix,
                    rest,
                    i,
                    c
                );
                return (Some(prefix), rest);
            }
        }
        // If all characters are in the set, return None and the original input
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;

    use super::*;
    use crate::str_parser::filters::DIGITS;

    /// Test when input starts with chars in the set and stops at first not in
    /// set. Should return the prefix of chars in the set and the rest.
    #[test]
    fn test_until_not_in_char_set_basic() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNotInCharSet(&DIGITS);
        let input = "123abc";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("123"));
        assert_eq!(rest, "abc");
    }

    /// Test when input starts with a char not in the set.
    /// Should return an empty prefix and the full input as rest.
    #[test]
    fn test_until_not_in_char_set_first_char_not_in_set() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNotInCharSet(&DIGITS);
        let input = "a123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some(""));
        assert_eq!(rest, "a123");
    }

    /// Test when all input is in the set (should return None).
    /// No character outside the set, so no split.
    #[test]
    fn test_until_not_in_char_set_all_in_set() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNotInCharSet(&DIGITS);
        let input = "123456";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "123456");
    }

    /// Test with empty input.
    /// Should return None and empty rest.
    #[test]
    fn test_until_not_in_char_set_empty_input() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNotInCharSet(&DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    /// Test with unicode character set.
    /// Should match all consecutive chars in the set, then split.
    #[test]
    fn test_until_not_in_char_set_unicode() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let filter: CharSetFilter<2> = CharSetFilter::from_str("好你")?;
        let rule = UntilNotInCharSet(&filter);
        let input = "你好世界";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
