use crate::str_parser::filters::IFilter;

pub struct CharSetFilter<const N: usize> {
    table: [bool; N],
}

impl<const N: usize> CharSetFilter<N> {
    /// Build from a string literal. Panics at **compile time**
    /// if you give it a char outside the range `0..N`.
    pub const fn from_str(chars: &str) -> Self {
        let mut tbl = [false; N];
        let bytes = chars.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            let idx = bytes[i] as usize;  
            assert!(idx < N, "char outside table range");
            tbl[idx] = true;
            i += 1;
        }
        Self { table: tbl }
    }

    /// Plain helper so you can call it outside of the trait.

    pub const fn contains(&self, c: char) -> bool {
        let code = c as usize;
        code < N && self.table[code]
    }
}
impl<const N: usize> IFilter<&char> for CharSetFilter<N> {
    fn name(&self) -> &str { "Filter Table" }
    fn filter(&self, input: &char) -> bool { self.contains(*input) }
}
/// Digits ‘0’–‘9’. The table only needs 58 bytes (0..58).
pub const DIGITS: CharSetFilter<58> = CharSetFilter::from_str("0123456789");

///ASCII ALL
/// Printable ASCII 0x00–0x7F (128 chars) ⇒ table size 128 bytes for simplicity.
pub const ASCII_ALL: CharSetFilter<128> = CharSetFilter::from_str(
    "\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
     \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
     \x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\
     0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\x7F",
);

///ASCII whitespace
/// ASCII whitespace characters: space, tab, line feed, carriage return, form
/// feed. The table needs 33 bytes (0..33) to cover up to ASCII 0x20 (space).
pub const ASCII_WHITESPACE: CharSetFilter<33> = CharSetFilter::from_str(" \t\n\r\x0C");

/// ASCII letters ‘A’–‘Z’, ‘a’–‘z’. The table needs 123 bytes (0..123) to cover
/// up to 'z'.
pub const ASCII_LETTERS: CharSetFilter<123> =
    CharSetFilter::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");

/// ASCII letters and digits. The table needs 123 bytes (0..123) to cover up to
/// 'z'.
pub const ASCII_LETTERS_DIGITS: CharSetFilter<123> =
    CharSetFilter::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
