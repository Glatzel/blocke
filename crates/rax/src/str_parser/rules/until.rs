use super::IStrFlowRule;
use crate::str_parser::IRule;

/// Rule to extract everything from the input string up to (but not including)
/// the first occurrence of a specified delimiter substring.
/// Returns a tuple of (prefix, rest) if the delimiter is found,
/// otherwise returns None.
/// If `include` is true, the delimiter is included in the prefix.
pub struct Until {
    pub delimiter: &'static str,
    pub include: bool,
}

impl IRule for Until {
    fn name(&self) -> &str { "Until" }
}

impl<'a> IStrFlowRule<'a> for Until {
    type Output = &'a str;
    /// Applies the Until rule to the input string.
    /// If the delimiter is found, returns the substring before the delimiter
    /// and the rest of the string (starting with the delimiter).
    /// If `include` is true, the delimiter is included in the prefix.
    /// Otherwise, returns None.
    fn apply(&self, input: &'a str) -> (Option<&'a str>, &'a str) {
        // Log the input and delimiter at trace level.
        clerk::trace!(
            "Until rule: input='{}', delimiter='{}', include={}",
            input,
            self.delimiter,
            self.include
        );
        match input.find(self.delimiter) {
            Some(idx) => {
                if self.include {
                    let end = idx + self.delimiter.len();
                    clerk::debug!(
                        "Until rule matched (include): prefix='{}', rest='{}'",
                        &input[..end],
                        &input[end..]
                    );
                    (Some(&input[..end]), &input[end..])
                } else {
                    clerk::debug!(
                        "Until rule matched: prefix='{}', rest='{}'",
                        &input[..idx],
                        &input[idx..]
                    );
                    (Some(&input[..idx]), &input[idx..])
                }
            }
            None => {
                clerk::debug!(
                    "Until rule did not match: delimiter '{}' not found in '{}'",
                    self.delimiter,
                    input
                );
                (None, input)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use clerk::init_log_with_level;
    use tracing::level_filters::LevelFilter;

    use super::*;

    #[test]
    fn test_until_str_output_not_include() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = Until {
            delimiter: ";",
            include: false,
        };
        let input = "hello;world";

        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("hello"));
        assert_eq!(rest, ";world");
    }

    #[test]
    fn test_until_str_output_include() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = Until {
            delimiter: ";",
            include: true,
        };
        let input = "hello;world";

        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("hello;"));
        assert_eq!(rest, "world");
    }

    #[test]
    fn test_until_parse_fail() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = Until {
            delimiter: ",",
            include: false,
        };
        let input = "abc rest";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "abc rest");
    }

    #[test]
    fn test_until_at_start_not_include() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = Until {
            delimiter: "-",
            include: false,
        };
        let input = "-start";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some(""));
        assert_eq!(rest, "-start");
    }

    #[test]
    fn test_until_at_start_include() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = Until {
            delimiter: "-",
            include: true,
        };
        let input = "-start";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, Some("-"));
        assert_eq!(rest, "start");
    }

    #[test]
    fn test_until_empty_input() {
        init_log_with_level(LevelFilter::TRACE);
        let rule = Until {
            delimiter: ",",
            include: false,
        };
        let input = "";
        let (prefix, rest) = rule.apply(input);
        assert_eq!(prefix, None);
        assert_eq!(rest, "");
    }
}
