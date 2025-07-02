use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

/// Rule to extract a fixed number of bytes from the input string.
/// Returns a tuple of (prefix, rest) if enough bytes are present and the split
/// is on a valid UTF-8 boundary, otherwise returns None.
pub struct ByteCount(pub usize);

impl IRule for ByteCount {
    fn name(&self) -> &str { "byte count" }
}

impl<'a> IStrFlowRule<'a, &'a str> for ByteCount {
    /// Applies the ByteCount rule to the input string.
    /// If the input contains at least `self.0` bytes and the split is on a
    /// valid UTF-8 boundary, returns the first `self.0` bytes and the rest
    /// of the string. Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Log the input and the requested byte count at trace level.
        clerk::trace!("ByteCount rule: input='{}', byte_count={}", input, self.0);

        match input.get(..self.0) {
            Some(out) => {
                let rest = &input[self.0..];
                clerk::debug!("ByteCount: matched prefix='{}', rest='{}'", out, rest);
                (Some(out), rest)
            }
            None => {
                clerk::debug!(
                    "ByteCount: not enough bytes or invalid UTF-8 boundary for count {} in '{}'",
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
    fn test_count_exact_length() {
        init_log();
        let rule = ByteCount(4);
        let input = "test";
        let result = rule.apply(input);
        assert_eq!(result, (Some("test"), ""));
    }

    #[test]
    fn test_count_less_than_length() {
        init_log();
        let rule = ByteCount(2);
        let input = "hello";
        let result = rule.apply(input);
        assert_eq!(result, (Some("he"), "llo"));
    }

    #[test]
    fn test_count_more_than_length() {
        init_log();
        let rule = ByteCount(10);
        let input = "short";
        let result = rule.apply(input);
        assert_eq!(result, (None, "short"));
    }

    #[test]
    fn test_count_zero() {
        init_log();
        let rule = ByteCount(0);
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, (Some(""), "abc"));
    }

    #[test]
    fn test_count_empty_input() {
        let rule = ByteCount(0);
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, (Some(""), ""));
    }

    #[test]
    fn test_count_non_ascii() {
        let rule = ByteCount(2);
        let input = "你好世界";

        // Each Chinese character is 3 bytes, but .get(..n) is by byte index, not char
        // index. So Count(2) will get the first 2 bytes, which is not a valid
        // UTF-8 boundary. This should return None.
        let result = rule.apply(input);
        assert_eq!(result, (None, "你好世界"));
    }
}
