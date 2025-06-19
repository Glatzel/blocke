use super::IStrFlowRule;
use crate::str_parser::rules::IRule;

pub struct Char<'a>(pub &'a char);
impl<'a> IRule for Char<'a> {
    fn name(&self) -> &str { "char" }
}
impl<'a> IStrFlowRule<'a, char> for Char<'a> {
    fn apply(&self, input: &'a str) -> Option<(char, &'a str)> {
        clerk::trace!("Char rule: input='{}', expected='{}'", input, self.0);
        let mut chars = input.char_indices();
        let (_, out) = chars.next()?; // first char's byte offset (0)
        if &out == self.0 {
            let (end, _) = chars.next().unwrap_or((input.len(), '\0')); // second char or end of string
            clerk::debug!("Char rule matched: '{}', rest='{}'", out, &input[end..]);
            Some((out, &input[end..]))
        } else {
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
