use super::IStrTakeRule;
use crate::str_parser::rules::IRule;

pub struct Char<'a>(pub &'a char);
impl<'a> IRule for Char<'a> {
    fn name(&self) -> &str { "char" }
}
impl<'a> IStrTakeRule<'a, char> for Char<'a> {
    fn apply_take_rule(&self, input: &'a str) -> Option<(char, &'a str)> {
        let mut chars = input.char_indices();
        match chars.next() {
            Some((i, c)) => {
                if self.0 == &c {
                    Some((c, &input[i + 1..]))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_match() {
        let rule = Char(&'a');
        let input = "a123";
        let result = rule.apply_take_rule(input);
        assert_eq!(result, Some(('a', "123")));
    }

    #[test]
    fn test_char_no_match() {
        let rule = Char(&'d');
        let input = "abc";
        let result = rule.apply_take_rule(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        let rule = Char(&'a');
        let input = "";
        let result = rule.apply_take_rule(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        let rule = Char(&'你');
        let input = "你好";
        let result = rule.apply_take_rule(input);
        assert_eq!(result, Some(('你', "你好")));
    }
}
