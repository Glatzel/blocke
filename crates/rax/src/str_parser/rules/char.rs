use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

/// Rule to match a specific character at the start of the input string.
/// If the first character matches the expected character, returns a tuple of
/// (matched_char, rest_of_input). Otherwise, returns None.
pub struct Char<'a>(pub &'a char);

impl<'a> IRule for Char<'a> {
    fn name(&self) -> &str { "char" }
}

impl<'a> IStrFlowRule<'a, char> for Char<'a> {
    /// Applies the Char rule to the input string.
    /// If the first character matches `self.0`, returns the character and the
    /// rest of the string. Otherwise, returns None.
    fn apply(&self, input: &'a str) -> Option<(char, &'a str)> {
        // Log the input and the expected character at trace level.
        clerk::trace!("Char rule: input='{}', expected='{}'", input, self.0);
        let mut chars = input.char_indices();
        
        // Get the first character and its byte offset.
        let (_, out) = chars.next()?; // first char's byte offset (0)
        if &out == self.0 {
            // If the character matches, find the next char boundary (or end of string).
            let (end, _) = chars.next().unwrap_or((input.len(), '\0')); // second char or end of string
            clerk::debug!("Char rule matched: '{}', rest='{}'", out, &input[end..]);
            Some((out, &input[end..]))
        } else {
            // If the character does not match, log and return None.
            clerk::debug!(
                "Char rule did not match: found '{}', expected '{}'",
                out,
                self.0
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::init_log;

    use super::*;

    #[test]
    fn test_char_match() {
        init_log();
        let rule = Char(&'a');
        let input = "a123";
        let result = rule.apply(input);
        assert_eq!(result, Some(('a', "123")));
    }

    #[test]
    fn test_char_no_match() {
        init_log();
        let rule = Char(&'d');
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        init_log();
        let rule = Char(&'a');
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        init_log();
        let rule = Char(&'你');
        let input = "你好";
        let result = rule.apply(input);
        assert_eq!(result, Some(('你', "好")));
    }
}
