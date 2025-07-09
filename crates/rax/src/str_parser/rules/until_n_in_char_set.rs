use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};
use crate::str_parser::rules::UntilMode;

/// Rule that extracts a prefix from the input string up to (but not including)
/// the position where N or more characters in the set have been seen.
/// Returns a tuple of (prefix, rest) where `prefix` contains all characters
/// up to the N-th character in the set, and `rest` is the remainder of the
/// string starting from that character. If fewer than N characters in the set
/// are found, returns (None, input).
/// If `include` is true, the N-th matched character is included in the prefix.
pub struct UntilNInCharSet<'a, const N: usize, const M: usize> {
    pub filter: &'a CharSetFilter<M>,
    pub mode: UntilMode,
}

impl<'a, const N: usize, const M: usize> IRule for UntilNInCharSet<'a, N, M> {
    fn name(&self) -> &str { "UntilNInCharSet" }
}

impl<'a, const N: usize, const M: usize> IStrFlowRule<'a> for UntilNInCharSet<'a, N, M> {
    type Output = &'a str;

    /// Applies the rule to the input string, returning the prefix up to the
    /// N-th character in the set, and the rest of the string starting from
    /// that character. If fewer than N characters in the set are found,
    /// returns (None, input).
    /// If `include` is true, the N-th matched character is included in the
    /// prefix.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        let mut count = 0;
        let mut start_idx = None;
        let mut end_idx = None;

        // Iterate over each character and its byte index in the input string
        for (i, c) in input.char_indices() {
            if self.filter.filter(&c) {
                count += 1;
                if count == N {
                    // If include, split after the N-th matched character
                    let end_of_char = i + c.len_utf8();
                    start_idx = Some(match self.mode {
                        UntilMode::Discard | UntilMode::KeepRight => i,
                        UntilMode::KeepLeft => end_of_char,
                    });
                    end_idx = Some(match self.mode {
                        UntilMode::Discard | UntilMode::KeepLeft => end_of_char,
                        UntilMode::KeepRight => i,
                    });
                    break;
                }
            }
        }

        if let Some(start_idx) = start_idx
            && let Some(end_idx) = end_idx
        {
            let prefix = &input[..start_idx];
            let rest = &input[end_idx..];
            clerk::debug!(
                "UntilNInCharSet: prefix='{}', rest='{}', N={}, mode={}",
                prefix,
                rest,
                N,
                self.mode
            );
            (Some(prefix), rest)
        } else {
            (None, input)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clerk::init_log_with_level;
    use tracing_subscriber::filter::LevelFilter;

    use super::*;
    use crate::str_parser::filters::DIGITS;

    #[test]
    fn test_until_n_in_char_set_discard() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<2, 10> {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Should split before the second digit ('2'), so prefix is "a1b", rest is "2c3"
        assert_eq!(prefix, Some("a1b"));
        assert_eq!(rest, "c3");
    }

    #[test]
    fn test_until_n_in_char_set_keep_left() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<2, 10> {
            filter: &DIGITS,
            mode: UntilMode::KeepLeft,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Should split after the second digit ('2'), so prefix is "a1b2", rest is "c3"
        assert_eq!(prefix, Some("a1b2"));
        assert_eq!(rest, "c3");
    }

    #[test]
    fn test_until_n_in_char_set_keep_right() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<2, 10> {
            filter: &DIGITS,
            mode: UntilMode::KeepRight,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Should split before the second digit ('2'), so prefix is "a1b", rest is "2c3"
        assert_eq!(prefix, Some("a1b"));
        assert_eq!(rest, "2c3");
    }

    #[test]
    fn test_until_n_in_char_set_not_enough_matches() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<4, 10> {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "a1b2c3";
        let (prefix, rest) = rule.apply(input);
        // Only 3 digits, so should return None and the original input
        assert_eq!(prefix, None);
        assert_eq!(rest, "a1b2c3");
    }

    #[test]
    fn test_until_n_in_char_set_empty_input() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilNInCharSet::<1, 10> {
            filter: &DIGITS,
            mode: UntilMode::Discard,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_until_n_in_char_set_unicode_keep_left() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let filter: CharSetFilter<3> = CharSetFilter::from_str("你世好")?;
        let rule = UntilNInCharSet::<2, 3> {
            filter: &filter,
            mode: UntilMode::KeepLeft,
        };
        let input = "你好世界";
        let (prefix, rest) = rule.apply(input);
        // Should split after the second match ('世'), so prefix is "你好世", rest is
        // "界"
        assert_eq!(prefix, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
