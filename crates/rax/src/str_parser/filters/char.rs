use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::str_parser::filters::IFilter;
pub static DIGITS: LazyLock<HashSet<char>> = LazyLock::new(|| ('0'..='9').collect());
pub static ASCII: LazyLock<HashSet<char>> =
    LazyLock::new(|| ('a'..='z').chain('A'..='Z').collect());
pub struct FilterChar<'a> {
    char_set: Cow<'a, HashSet<char>>,
}
impl<'a> IFilter<&char> for FilterChar<'a> {
    fn name(&self) -> &str { "Filter Char" }

    fn filter(&self, input: &char) -> bool { self.char_set.contains(input) }
}

impl<'a> FilterChar<'a> {
    pub fn from_str(char_set: &str) -> Self {
        Self {
            char_set: Cow::Owned(char_set.chars().collect()),
        }
    }
    pub fn digits() -> Self {
        Self {
            char_set: Cow::Borrowed(&DIGITS),
        }
    }
    pub fn ascii() -> Self {
        Self {
            char_set: Cow::Borrowed(&ASCII),
        }
    }
}
