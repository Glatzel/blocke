use super::IStrFlowRule;
use crate::str_parser::IRule;

pub struct Until<'a>(pub &'a str);
impl<'a> IRule for Until<'a> {
    fn name(&self) -> &str { "Until" }
}
impl<'a> IStrFlowRule<'a, &'a str> for Until<'a> {
    fn apply(&self, input: &'a str) -> Option<(&'a str, &'a str)> {
        match input.find(self.0) {
            Some(idx) => Some((&input[..idx], &input[idx..])),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_until_str_output() {
        // O = &str is not supported because FromStr is not implemented for &str
        // So we use String as output type
        let rule = Until(";");
        let input = "hello;world";

        let result = rule.apply(input);
        assert_eq!(result, Some(("hello", ";world")));
    }

    #[test]
    fn test_until_parse_fail() {
        let rule = Until(",");
        let input = "abc rest";
        let result = rule.apply(input);
        assert!(result.is_none());
    }
    #[test]
    fn test_until_at_start() {
        let rule = Until("-");
        let input = "-start";
        let result = rule.apply(input);
        assert_eq!(result, Some(("", "-start")));
    }

    #[test]
    fn test_until_empty_input() {
        let rule = Until(",");
        let input = "";

        let result = rule.apply(input);
        assert_eq!(result, None);
    }
}
