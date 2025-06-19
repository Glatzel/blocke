use crate::str_parser::filters::IFilter;

pub struct Char<'a>(&'a crate::str_parser::filters::FilterChar<'a>);
impl<'a> super::IRule<'a, char> for Char<'a> {
    fn name(&self) -> &str { "char" }
    fn apply_rule(&self, input: &'a str) -> Option<(char, &'a str)> {
        let mut chars = input.char_indices();
        match chars.next() {
            Some((i, c)) => {
                if self.0.filter(&c) {
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
    use crate::str_parser::filters::FilterChar;
    use crate::str_parser::rules::IRule;

    #[test]
    fn test_char_match() {
        let filter = FilterChar::ascii();
        let rule = Char(&filter);
        let input = "a123";
        let result = rule.apply_rule(input);
        assert_eq!(result, Some(('a', "a123")));
    }

    #[test]
    fn test_char_no_match() {
        let filter = FilterChar::digits();
        let rule = Char(&filter);
        let input = "abc";
        let result = rule.apply_rule(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        let filter = FilterChar::ascii();
        let rule = Char(&filter);
        let input = "";
        let result = rule.apply_rule(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        let filter = FilterChar::from_str("你");
        let rule = Char(&filter);
        let input = "你好";
        let result = rule.apply_rule(input);
        assert_eq!(result, Some(('你', "你好")));
    }
}
