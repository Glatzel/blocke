use super::IStrFlowRule;
use crate::str_parser::StrParserContext;

pub struct ByteCount(usize);
impl<'a> IStrFlowRule<'a, &'a str> for ByteCount {
    fn name(&self) -> &str { "byte count" }
    fn apply(&self, _: &StrParserContext, input: &'a str) -> Option<(&'a str, &'a str)> {
        match input.get(..self.0) {
            Some(out) => {
                let rest = &input[self.0..];
                Some((out, rest))
            }
            None => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_exact_length() {
        let rule = ByteCount(4);
        let input = "test";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, Some(("test", "")));
    }

    #[test]
    fn test_count_less_than_length() {
        let rule = ByteCount(2);
        let input = "hello";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, Some(("he", "llo")));
    }

    #[test]
    fn test_count_more_than_length() {
        let rule = ByteCount(10);
        let input = "short";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, None);
    }

    #[test]
    fn test_count_zero() {
        let rule = ByteCount(0);
        let input = "abc";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, Some(("", "abc")));
    }

    #[test]
    fn test_count_empty_input() {
        let rule = ByteCount(0);
        let input = "";
        let ctx = StrParserContext::new(input);
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, Some(("", "")));
    }

    #[test]
    fn test_count_non_ascii() {
        let rule = ByteCount(2);
        let input = "你好世界";
        let ctx = StrParserContext::new(input);
        // Each Chinese character is 3 bytes, but .get(..n) is by byte index, not char
        // index. So Count(2) will get the first 2 bytes, which is not a valid
        // UTF-8 boundary. This should return None.
        let result = rule.apply(&ctx, ctx.full);
        assert_eq!(result, None);
    }
}
