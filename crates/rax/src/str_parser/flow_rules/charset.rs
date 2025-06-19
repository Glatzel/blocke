use super::IStrFlowRule;
use crate::str_parser::StrParserContext;
use crate::str_parser::filters::IFilter;

pub struct CharSet<'a>(&'a crate::str_parser::filters::FilterCharSet<'a>);
impl<'a> IStrFlowRule<'a, char> for CharSet<'a> {
    fn name(&self) -> &str { "char" }
    fn apply(&self, _: &StrParserContext, input: &'a str) -> Option<(char, &'a str)> {
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
    use crate::str_parser::filters::FilterCharSet;

    #[test]
    fn test_char_match() {
        let filter = FilterCharSet::ascii();
        let rule = CharSet(&filter);
        let input = "a123";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, Some(('a', "a123")));
    }

    #[test]
    fn test_char_no_match() {
        let filter = FilterCharSet::digits();
        let rule = CharSet(&filter);
        let input = "abc";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        let filter = FilterCharSet::ascii();
        let rule = CharSet(&filter);
        let input = "";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        let filter = FilterCharSet::from_str("你");
        let rule = CharSet(&filter);
        let input = "你好";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, Some(('你', "你好")));
    }
}
