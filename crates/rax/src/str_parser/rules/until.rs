use super::IStrFlowRule;
use crate::str_parser::IRule;

/// Rule to extract everything from the input string up to (but not including)
/// the first occurrence of a specified delimiter substring.
/// Returns a tuple of (prefix, rest) if the delimiter is found,
/// otherwise returns None.
pub struct Until<'a>(pub &'a str);

impl<'a> IRule for Until<'a> {
    fn name(&self) -> &str { "Until" }
}

impl<'a> IStrFlowRule<'a, &'a str> for Until<'a> {
    /// Applies the Until rule to the input string.
    /// If the delimiter is found, returns the substring before the delimiter
    /// and the rest of the string (starting with the delimiter).
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        // Log the input and delimiter at trace level.
        clerk::trace!("Until rule: input='{}', delimiter='{}'", input, self.0);
        match input.find(self.0) {
            Some(idx) => {
                clerk::debug!(
                    "Until rule matched: prefix='{}', rest='{}'",
                    &input[..idx],
                    &input[idx..]
                );
                Some((&input[..idx], &input[idx..]))
            }
            None => {
                clerk::debug!(
                    "Until rule did not match: delimiter '{}' not found in '{}'",
                    self.0,
                    input
                );
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::init_log;

    use super::*;

    #[test]
    fn test_until_str_output() {
        // O = &str is not supported because FromStr is not implemented for &str
        // So we use String as output type
        init_log();
        let rule = Until(";");
        let input = "hello;world";

        let result = rule.apply(input);
        assert_eq!(result, Some(("hello", ";world")));
    }

    #[test]
    fn test_until_parse_fail() {
        // Test when the delimiter is not found in the input.
        init_log();
        let rule = Until(",");
        let input = "abc rest";
        let result = rule.apply(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_until_at_start() {
        // Test when the delimiter is at the start of the input.
        init_log();
        let rule = Until("-");
        let input = "-start";
        let result = rule.apply(input);
        assert_eq!(result, Some(("", "-start")));
    }

    #[test]
    fn test_until_empty_input() {
        // Test with an empty input string.
        init_log();
        let rule = Until(",");
        let input = "";

        let result = rule.apply(input);
        assert_eq!(result, None);
    }
}
