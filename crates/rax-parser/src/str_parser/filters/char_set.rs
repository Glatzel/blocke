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
    fn filter(&self, input: &char) -> bool { self.contains(*input) }
}
impl<const N: usize> CharSetFilter<N> {
    pub fn from_str(s: &str) -> Self {
        let mut chars = [0 as char; N];
        let mut i = 0;
        for c in s.chars() {
            if i < N {
                chars[i] = c;
                i += 1;
            } else {
                panic!("String too long for CharSet, expected {} but got {}", N, i);
            }
        }
        if i != N {
            panic!(
                "String length does not match CharSet size, expected {} but got {}",
                N, i
            );
        }
        Self::new(chars)
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
    // digits
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', // uppercase letters
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', // lowercase letters
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
]);
