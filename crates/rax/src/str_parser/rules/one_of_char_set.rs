use super::IStrFlowRule;
use crate::str_parser::IRule;
use crate::str_parser::filters::IFilter;

pub struct OneOfCharSet<'a>(&'a crate::str_parser::filters::FilterCharSet<'a>);
impl<'a> IRule for OneOfCharSet<'a> {
    fn name(&self) -> &str { "OneOfCharSet" }
}
impl<'a> IStrFlowRule<'a, char> for OneOfCharSet<'a> {
    fn apply(&self, input: &'a str) -> Option<(char, &'a str)> {
        clerk::trace!("OneOfCharSet rule: input='{}'", input);
        let (_, c) = input.char_indices().next()?; // safely unwrap with ?
        if self.0.filter(&c) {
            let next_i = input.char_indices().nth(1).map_or(input.len(), |(j, _)| j);
            clerk::debug!("OneOfCharSet matched: '{}', rest='{}'", c, &input[next_i..]);
            Some((c, &input[next_i..]))
        } else {
            clerk::debug!("OneOfCharSet did not match: found '{}', not in set", c);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::init_log;

    use super::*;
    use crate::str_parser::filters::FilterCharSet;

    #[test]
    fn test_char_match() {
        init_log();
        let filter = FilterCharSet::ascii();
        let rule = OneOfCharSet(&filter);
        let input = "a123";
        let result = rule.apply(input);
        assert_eq!(result, Some(('a', "123")));
    }

    #[test]
    fn test_char_no_match() {
        init_log();
        let filter = FilterCharSet::digits();
        let rule = OneOfCharSet(&filter);
        let input = "abc";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_empty_input() {
        init_log();
        let filter = FilterCharSet::ascii();
        let rule = OneOfCharSet(&filter);
        let input = "";
        let result = rule.apply(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_char_unicode() {
        init_log();
        let filter = FilterCharSet::from_string("你");
        let rule = OneOfCharSet(&filter);
        let input = "你好";
        let result = rule.apply(input);
        assert_eq!(result, Some(('你', "好")));
    }
}
