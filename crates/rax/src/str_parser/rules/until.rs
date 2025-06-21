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
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Log the input and delimiter at trace level.
        clerk::trace!("Until rule: input='{}', delimiter='{}'", input, self.0);
        match input.find(self.0) {
            Some(idx) => {
                clerk::debug!(
                    "Until rule matched: prefix='{}', rest='{}'",
                    &input[..idx],
                    &input[idx..]
                );
                (Some(&input[..idx]), &input[idx..])
            }
            None => {
                clerk::debug!(
                    "Until rule did not match: delimiter '{}' not found in '{}'",
                    self.0,
                    input
                );
                (None, input)
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
        init_log();
        let rule = Until(";");
        let input = "hello;world";

        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("hello"));
        assert_eq!(rest, ";world");
    }

    #[test]
    fn test_until_parse_fail() {
        init_log();
        let rule = Until(",");
        let input = "abc rest";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "abc rest");
    }

    #[test]
    fn test_until_at_start() {
        init_log();
        let rule = Until("-");
        let input = "-start";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "-start");
    }

    #[test]
    fn test_until_empty_input() {
        init_log();
        let rule = Until(",");
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }
}
