use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

/// Rule to extract a fixed number of characters from the input string.
/// Returns a tuple of (prefix, rest) if enough characters are present,
/// otherwise returns None.
pub struct CharCount(usize);

impl IRule for CharCount {
    fn name(&self) -> &str { "CharCount" }
}

impl<'a> IStrFlowRule<'a, &'a str> for CharCount {
    /// Applies the CharCount rule to the input string.
    /// If the input contains at least `self.0` characters, returns
    /// the first `self.0` characters and the rest of the string.
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        // Log the input and the requested character count at trace level.
        clerk::trace!("CharCount rule: input='{}', count={}", input, self.0);

        // If count is zero, return empty prefix and full input.
        if self.0 == 0 {
            clerk::debug!("CharCount: count is zero, returning empty prefix and full input.");
            return Some(("", input));
        }

        // Count the number of characters in the input.
        let indices = input.char_indices();
        let length = indices.count();

        // If count matches input length, return the whole input as prefix.
        if self.0 == length {
            clerk::debug!("CharCount: count matches input length, returning whole input.");
            return Some((input, ""));
        }

        // Iterate over char boundaries to find the split point.
        for (count, (idx, _)) in input.char_indices().by_ref().enumerate() {
            if count == self.0 {
                // Found the split point at the requested character count.
                clerk::debug!(
                    "CharCount: found split at char {}, byte idx {}: prefix='{}', rest='{}'",
                    count,
                    idx,
                    &input[..idx],
                    &input[idx..]
                );
                return Some((&input[..idx], &input[idx..]));
            }
        }

        // Not enough characters in the input.
        clerk::warn!(
            "CharCount: not enough chars in input (needed {}, found {})",
            self.0,
            length
        );
        None
    }
}

#[cfg(test)]
mod tests {
    use test_utils::init_log;

    use super::*;

    #[test]
    fn test_count_exact_length() {
        init_log();
        let rule = CharCount(4);
        let input = "test";
        let result = rule.apply(input);
        assert_eq!(result, Some(("test", "")));
    }

    #[test]
    fn test_count_less_than_length() {
        init_log();
        let rule = CharCount(2);
        let input = "hello";
        let result = rule.apply(input);
        assert_eq!(result, Some(("he", "llo")));
    }

    #[test]
    fn test_count_more_than_length() {
        init_log();
        let rule = CharCount(10);
        let input = "short";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_count_zero() {
        init_log();
        let rule = CharCount(0);
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, Some(("", "abc")));
    }

    #[test]
    fn test_count_empty_input() {
        init_log();
        let rule = CharCount(0);
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, Some(("", "")));
    }

    #[test]
    fn test_count_non_ascii() {
        init_log();
        let rule = CharCount(2);
        let input = "你好世界";
        // Should return first 2 chars ("你", "好") and the rest ("世界")
        let result = rule.apply(input);
        assert_eq!(result, Some(("你好", "世界")));
    }
}
