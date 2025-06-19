use std::str::FromStr;

pub struct Until<'a>(&'a str);
impl<'a, O> super::IRule<'a, O> for Until<'a>
where
    O: FromStr,
{
    fn name(&self) -> &str { "until" }
    fn apply_rule(&self, input: &'a str) -> Option<(O, &'a str)> {
        match input.find(self.0) {
            Some(idx) => match input[..idx].parse::<O>() {
                Ok(out) => Some((out, &input[idx..])),
                Err(_) => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::str_parser::rules::IRule;

    #[test]
    fn test_until_str_output() {
        // O = &str is not supported because FromStr is not implemented for &str
        // So we use String as output type
        let rule = Until(";");
        let input = "hello;world";
        let result: Option<(String, &str)> = rule.apply_rule(input);
        assert_eq!(result, Some(("hello".to_string(), ";world")));
    }

    #[test]
    fn test_until_int_output() {
        let rule = Until(",");
        let input = "123,rest";
        let result: Option<(i32, &str)> = rule.apply_rule(input);
        assert_eq!(result, Some((123, ",rest")));
    }

    #[test]
    fn test_until_parse_fail() {
        let rule = Until(",");
        let input = "abc,rest";
        let result: Option<(bool, &str)> = rule.apply_rule(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_until_not_found() {
        let rule = Until("|");
        let input = "no delimiter here";
        let result: Option<(bool, &str)> = rule.apply_rule(input);
        assert!(result.is_none());
    }

    #[test]
    fn test_until_at_start() {
        let rule = Until("-");
        let input = "-start";
        let result: Option<(String, &str)> = rule.apply_rule(input);
        assert_eq!(result, Some(("".to_string(), "-start")));
    }

    #[test]
    fn test_until_empty_input() {
        let rule = Until(",");
        let input = "";
        let result: Option<(bool, &str)> = rule.apply_rule(input);
        assert_eq!(result, None);
    }
}
