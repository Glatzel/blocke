use crate::str_parser::StrParserContext;

use super::IStrFlowRule;

pub struct Char<'a>(&'a char);
impl<'a> IStrFlowRule<'a, char> for Char<'a> {
    fn name(&self) -> &str { "char" }
    fn apply(&self,_: &StrParserContext, input: &'a str) -> Option<(char, &'a str)> {
        let mut chars = input.char_indices();
        match chars.next() {
            Some((i, c)) => {
                if self.0 == &c {
                    Some((c, &input[i..]))
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
    use crate::str_parser::filters::FilterCharSet;

    #[test]
    fn test_char_match() {
        let rule = Char(&'a');
        let input = "a123";
        let result = rule.apply(input);
        assert_eq!(result, Some(('a', "a123")));
    }

    #[test]
    fn test_char_no_match() {
        let rule = Char(&'d');
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        let rule = Char(&'a');
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        let rule = Char(&'你');
        let input = "你好";
        let result = rule.apply(input);
        assert_eq!(result, Some(('你', "你好")));
    }
}
