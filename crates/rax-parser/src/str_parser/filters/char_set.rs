use std::str::FromStr;

use crate::str_parser::filters::IFilter;

/// A fixed, sorted list of characters.
/// `contains()` uses a const‑friendly binary search.
/// No nightly features required.
pub struct CharSetFilter<const N: usize> {
    table: [char; N],
}

impl<const N: usize> CharSetFilter<N> {
    /// The caller promises `table` is sorted and unique.
    pub const fn new(table: [char; N]) -> Self { Self { table } }

    /// Compile‑time binary search (O(log N)).
    pub const fn contains(&self, target: char) -> bool {
        let mut lo = 0;
        let mut hi = N;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mid_val = self.table[mid];
            if target == mid_val {
                return true;
            }
            if target < mid_val {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        false
    }
}
impl<const N: usize> IFilter<&char> for CharSetFilter<N> {
    fn name(&self) -> &str { "Char Set (array)" }
    fn filter(&self, input: &char) -> bool {
        clerk::trace!(
            "CharSetFilter: checking if '{}' is in the set {:?}",
            input,
            self.table
        );
        self.contains(*input)
    }
}
impl<const N: usize> FromStr for CharSetFilter<N> {
    type Err = miette::Report;

    fn from_str(s: &str) -> miette::Result<Self> {
        let mut chars = [0 as char; N];
        let mut i = 0;
        for c in s.chars() {
            if i < N {
                chars[i] = c;
                i += 1;
            } else {
                miette::bail!("String too long for CharSet, expected {} but got {}", N, i);
            }
        }
        if i != N {
            miette::bail!(
                "String length does not match CharSet size, expected {} but got {}",
                N,
                i
            );
        }
        Ok(Self::new(chars))
    }
}

/// Digits (10 items) – lookup is ~3 comparisons.
pub const DIGITS: CharSetFilter<10> =
    CharSetFilter::new(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

/// ASCII letters (52 items) – lookup is ~6 comparisons.
/// This includes both uppercase and lowercase letters.
pub const ASCII_LETTERS: CharSetFilter<52> = CharSetFilter::new([
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
]);

pub const ASCII_LETTERS_DIGITS: CharSetFilter<62> = CharSetFilter::new([
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_contains() {
        for c in '0'..='9' {
            assert!(super::DIGITS.contains(c), "DIGITS should contain '{}'", c);
        }
        assert!(!super::DIGITS.contains('a'));
        assert!(!super::DIGITS.contains(' '));
    }

    #[test]
    fn test_ascii_letters_contains() {
        for c in 'A'..='Z' {
            assert!(
                super::ASCII_LETTERS.contains(c),
                "ASCII_LETTERS should contain '{}'",
                c
            );
        }
        for c in 'a'..='z' {
            assert!(
                super::ASCII_LETTERS.contains(c),
                "ASCII_LETTERS should contain '{}'",
                c
            );
        }
        assert!(!super::ASCII_LETTERS.contains('0'));
        assert!(!super::ASCII_LETTERS.contains(' '));
    }

    #[test]
    fn test_ascii_letters_digits_contains() {
        for c in 'A'..='Z' {
            assert!(super::ASCII_LETTERS_DIGITS.contains(c));
        }
        for c in 'a'..='z' {
            assert!(super::ASCII_LETTERS_DIGITS.contains(c));
        }
        for c in '0'..='9' {
            assert!(super::ASCII_LETTERS_DIGITS.contains(c));
        }
        assert!(!super::ASCII_LETTERS_DIGITS.contains(' '));
        assert!(!super::ASCII_LETTERS_DIGITS.contains('-'));
    }

    #[test]
    fn test_from_str_digits() {
        let digits: CharSetFilter<10> = "0123456789".parse().unwrap();
        for c in '0'..='9' {
            assert!(digits.contains(c));
        }
    }

    #[test]
    fn test_from_str_too_short() {
        let result: Result<CharSetFilter<10>, _> = "01234567".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_too_long() {
        let result: Result<CharSetFilter<10>, _> = "01234567890".parse();
        assert!(result.is_err());
    }
}
