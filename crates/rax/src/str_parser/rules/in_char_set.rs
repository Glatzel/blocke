use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::IFilter;

pub struct InCharSet<'a>(&'a crate::str_parser::filters::FilterCharSet<'a>);
impl<'a> IRule for InCharSet<'a> {
    fn name(&self) -> &str { todo!() }
}
impl<'a> IStrFlowRule<'a, char> for InCharSet<'a> {
    fn apply(&self, input: &'a str) -> Option<(char, &'a str)> {
        let (_, c) = input.char_indices().next()?; // safely unwrap with ?
        if self.0.filter(&c) {
            let next_i = input.char_indices().nth(1).map_or(input.len(), |(j, _)| j);
            Some((c, &input[next_i..]))
        } else {
            None
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
        let rule = InCharSet(&filter);
        let input = "a123";
        let result = rule.apply(input);
        assert_eq!(result, Some(('a', "123")));
    }

    #[test]
    fn test_char_no_match() {
        let filter = FilterCharSet::digits();
        let rule = InCharSet(&filter);
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        let filter = FilterCharSet::ascii();
        let rule = InCharSet(&filter);
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        let filter = FilterCharSet::from_string("你");
        let rule = InCharSet(&filter);
        let input = "你好";
        let result = rule.apply(input);
        assert_eq!(result, Some(('你', "好")));
    }
}
