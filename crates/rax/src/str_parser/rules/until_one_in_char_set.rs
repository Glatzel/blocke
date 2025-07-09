use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule to extract everything from the input string up to (but not including)
/// the first occurrence of any character in the provided character set.
/// Returns a tuple of (prefix, rest) where `prefix` contains all characters
/// up to the first character in the set, and `rest` is the remainder of the
/// string starting from that character. If no character in the set is found,
/// returns (None, input).
pub struct UntilOneInCharSet<'a, const N: usize> {
    pub filter: &'a CharSetFilter<N>,
    pub include: bool,
}

impl<'a, const N: usize> IRule for UntilOneInCharSet<'a, N> {
    fn name(&self) -> &str { "UntilOneInCharSet" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilOneInCharSet<'a, N> {
    type Output = &'a str;

    /// Applies the UntilOneInCharSet rule to the input string.
    /// If a character in the set is found, returns the substring before the
    /// character and the rest of the string (starting with the character).
    /// If `include` is true, the matched character is included in the prefix.
    /// If `include` is false and the first character is in the set, returns
    /// None. If no character in the set is found, returns None and the
    /// original input.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        for (i, c) in input.char_indices() {
            if self.filter.filter(&c) {
                match (self.include, i == 0) {
                    (true, _) => {
                        // Include the matched character in the prefix
                        let prefix = &input[..i + c.len_utf8()];
                        let rest = &input[i + c.len_utf8()..];
                        clerk::debug!(
                            "UntilOneInCharSet(include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }
                    (false, true) => {
                        // Not include, and first char is in set
                        clerk::debug!(
                            "UntilOneInCharSet(not include): first char in set, returning None, input='{}'",
                            input
                        );
                        return (None, input);
                    }
                    (false, false) => {
                        // Not include, and not first char
                        let prefix = &input[..i];
                        let rest = &input[i..];
                        clerk::debug!(
                            "UntilOneInCharSet(not include): prefix='{}', rest='{}', i={}, c='{}'",
                            prefix,
                            rest,
                            i,
                            c
                        );
                        return (Some(prefix), rest);
                    }
                }
            }
        }
        // No character in the set found
        clerk::debug!(
            "UntilOneInCharSet: no match found, returning None, input='{}'",
            input
        );
        (None, input)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use clerk::init_log_with_level;
    use tracing_subscriber::filter::LevelFilter;

    use super::*;
    use crate::str_parser::filters::{ASCII_LETTERS_DIGITS, DIGITS};

    #[test]
    fn test_until_one_of_char_set_include_true() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let filter = CharSetFilter::<2>::from_str(",*")?;
        let rule = UntilOneInCharSet {
            filter: &filter,
            include: true,
        };
        let input = "0.7,1*38";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("0.7,"));
        assert_eq!(rest, "1*38");
        Ok(())
    }

    #[test]
    fn test_until_one_of_char_set_include_false_first_char() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            include: false,
        };
        let input = "1abc";
        let (matched, rest) = rule.apply(input);
        // First char is in set and include=false, should return None
        assert_eq!(matched, None);
        assert_eq!(rest, "1abc");
    }

    #[test]
    fn test_until_one_of_char_set_include_false_middle() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet {
            filter: &DIGITS,
            include: false,
        };
        let input = "abc123";
        let (matched, rest) = rule.apply(input);
        // Should split before first digit
        assert_eq!(matched, Some("abc"));
        assert_eq!(rest, "123");
    }

    #[test]
    fn test_until_one_of_char_set_no_match() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet {
            filter: &ASCII_LETTERS_DIGITS,
            include: true,
        };
        let input = "!@#$%^&*()";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, input);
    }

    #[test]
    fn test_until_one_of_char_set_empty_input() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet {
            filter: &ASCII_LETTERS_DIGITS,
            include: true,
        };
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, input);
    }

    #[test]
    fn test_until_one_of_char_set_unicode() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let filter: CharSetFilter<1> = CharSetFilter::from_str("好")?;
        let rule = UntilOneInCharSet {
            filter: &filter,
            include: true,
        };
        let input = "你好世界";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("你好"));
        assert_eq!(rest, "世界");
        Ok(())
    }
}
