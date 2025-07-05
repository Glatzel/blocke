use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::{CharSetFilter, IFilter};

/// Rule to extract everything from the input string up to (but not including)
/// the first occurrence of a specified delimiter substring.
/// Returns a tuple of (prefix, rest) if the delimiter is found,
/// otherwise returns None.
pub struct UntilOneInCharSet<'a, const N: usize>(pub &'a CharSetFilter<N>);

impl<'a, const N: usize> IRule for UntilOneInCharSet<'a, N> {
    fn name(&self) -> &str { "Until" }
}

impl<'a, const N: usize> IStrFlowRule<'a> for UntilOneInCharSet<'a, N> {
    type Output = &'a str;
    /// Applies the Until rule to the input string.
    /// If the delimiter is found, returns the substring before the delimiter
    /// and the rest of the string (starting with the delimiter).
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        for (i, c) in input.char_indices() {
            if self.0.filter(&c) {
                // If the character is in the set, return the prefix and the rest of the string
                let prefix = &input[..i];
                let rest = &input[i..];
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
        (None, input)
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;

    use super::*;
    use crate::str_parser::filters::{ASCII_LETTERS_DIGITS, DIGITS};
    #[test]
    fn test_until_one_of_char_set() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet(&ASCII_LETTERS_DIGITS);
        let input = "abc123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some(""));
        assert_eq!(rest, "abc123");

        let rule = UntilOneInCharSet(&DIGITS);
        let input = "abc123";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("abc"));
        assert_eq!(rest, "123");

        let filter = CharSetFilter::<2>::from_str(",*")?;
        let rule = UntilOneInCharSet::<2>(&filter);
        let input = "0.7,1*38";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("0.7"));
        assert_eq!(rest, ",1*38");
        Ok(())
    }
    #[test]
    fn test_until_one_of_char_set_no_match() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet(&ASCII_LETTERS_DIGITS);
        let input = "!@#$%^&*()";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, input);
    }
    #[test]
    fn test_until_one_of_char_set_empty_input() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = UntilOneInCharSet(&ASCII_LETTERS_DIGITS);
        let input = "";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, None);
        assert_eq!(rest, input);
    }

    #[test]
    fn test_until_one_of_char_set_unicode() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        let filter: CharSetFilter<1> = CharSetFilter::from_str("好")?;
        let rule = UntilOneInCharSet(&filter);
        let input = "你好世界";
        let (matched, rest) = rule.apply(input);
        assert_eq!(matched, Some("你"));
        assert_eq!(rest, "好世界");
        Ok(())
    }
}
