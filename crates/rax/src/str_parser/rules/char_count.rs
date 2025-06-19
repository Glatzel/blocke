use crate::str_parser::rules::IRule;

use super::IStrTakeRule;

pub struct CharCount(usize);
impl IRule for CharCount {
    fn name(&self) -> &str { "CharCount" }
}
impl<'a> IStrTakeRule<'a, &'a str> for CharCount {
    fn apply(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        if self.0 == 0 {
            return Some(("", input));
        }

        let indices = input.char_indices();
        let length = indices.count();
        if self.0 == length {
            return Some((input, ""));
        }

        for (count, (idx, _)) in input.char_indices().enumerate() {
            if count == self.0 {
                return Some((&input[..idx], &input[idx..]));
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_exact_length() {
        let rule = CharCount(4);
        let input = "test";
        let result = rule.apply(input);
        assert_eq!(result, Some(("test", "")));
    }

    #[test]
    fn test_count_less_than_length() {
        let rule = CharCount(2);
        let input = "hello";
        let result = rule.apply(input);
        assert_eq!(result, Some(("he", "llo")));
    }

    #[test]
    fn test_count_more_than_length() {
        let rule = CharCount(10);
        let input = "short";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_count_zero() {
        let rule = CharCount(0);
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, Some(("", "abc")));
    }

    #[test]
    fn test_count_empty_input() {
        let rule = CharCount(0);
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, Some(("", "")));
    }

    #[test]
    fn test_count_non_ascii() {
        let rule = CharCount(2);
        let input = "你好世界";
        // Each Chinese character is 3 bytes, but .get(..n) is by byte index, not char
        // index. So Count(2) will get the first 2 bytes, which is not a valid
        // UTF-8 boundary. This should return None.
        let result = rule.apply(input);
        assert_eq!(result, Some(("你好", "世界")));
    }
}
