use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule that extracts a prefix from the input string up to (but not including)
/// the position where N or more characters in the set have been seen.
/// Returns a tuple of (prefix, rest) where `prefix` contains all characters
/// up to the N-th character in the set, and `rest` is the remainder of the
/// string starting from that character. If fewer than N characters in the set
/// are found, returns (None, input).
pub struct UntilNInCharSet<'a, const N: usize, const M: usize>(pub &'a CharSetFilter<M>);

impl<'a, const N: usize, const M: usize> IRule for UntilNInCharSet<'a, N, M> {
    fn name(&self) -> &str { "UntilNInCharSet" }
}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for UntilNInCharSet<'a, N, M> {
    type Output = &'a str;

    /// Applies the rule to the input string, returning the prefix up to the
    /// N-th character in the set, and the rest of the string starting from
    /// that character. If fewer than N characters in the set are found,
    /// returns (None, input).
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        let mut count = 0;
        // Track the byte index where the N-th match occurs
        let mut split_idx = None;

        // Iterate over each character and its byte index in the input string
        for (i, c) in input.char_indices() {
            // If the character is in the set, increment the count
            if self.0.filter(&c) {
                count += 1;
                // If we've reached N matches, mark the split index
                if count == N {
                    split_idx = Some(i);
                    break;
                }
            }
        }

        // If we found at least N matches, split at the split_idx
        if let Some(idx) = split_idx {
            let prefix = &input[..idx];
            let rest = &input[idx..];
            clerk::debug!(
                "UntilNInCharSet matched: prefix='{}', rest='{}', idx={}, N={}",
                prefix,
                rest,
                idx,
                N
            );
            (Some(prefix), rest)
        } else {
            // Not enough matches, return None and the original input
            (None, input)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;

    use super::*;
    use crate::str_parser::filters::{DIGITS};

    /// Test when input contains at least M matches in the set.
    #[test]
    fn test_until_n_in_char_set_basic() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<2, 10>(&DIGITS);
        let input = "a1b2c3";
        let (matched, rest) = rule.apply(input);
        // Should split before the second digit ('2'), so prefix is "a1b"
        assert_eq!(matched, Some("a1b"));
        assert_eq!(rest, "2c3");
    }

    /// Test when input contains exactly M matches in the set.
    #[test]
    fn test_until_n_in_char_set_exact() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<3, 10>(&DIGITS);
        let input = "a1b2c3";
        let (matched, rest) = rule.apply(input);
        // Should split before the third digit ('3'), so prefix is "a1b2c"
        assert_eq!(matched, Some("a1b2c"));
        assert_eq!(rest, "3");
    }

    /// Test when input contains fewer than M matches in the set.
    #[test]
    fn test_until_n_in_char_set_not_enough_matches() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<4, 10>(&DIGITS);
        let input = "a1b2c3";
        let (matched, rest) = rule.apply(input);
        // Only 3 digits, so should return None and the original input
        assert_eq!(matched, None);
        assert_eq!(rest, "a1b2c3");
    }

    /// Test with empty input.
    #[test]
    fn test_until_n_in_char_set_empty_input() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<1, 10>(&DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, "");
    }

    /// Test with unicode character set.
    #[test]
    fn test_until_n_in_char_set_unicode() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let filter: CharSetFilter<2> = CharSetFilter::from_str("你世")?;
        let rule = UntilNInCharSet::<2, 2>(&filter);
        let input = "你好世界";
        let (matched, rest) = rule.apply(input);
        // Should split before the second match ('世'), so prefix is "你好"
        assert_eq!(matched, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
