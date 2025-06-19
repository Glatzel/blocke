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
