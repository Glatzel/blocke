use std::io::BufRead;

use miette::IntoDiagnostic;
pub trait IRaxReader {
    /// Reads the next line; returns `None` on EOF.
    fn read_line(&mut self) -> miette::Result<Option<String>>;
    fn read_lines_by_count(&mut self, count: usize) -> miette::Result<Vec<String>>;
}
pub struct RaxReader<R: BufRead> {
    inner: R,
    buf: String,
}

impl<R: BufRead> RaxReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: String::new(),
        }
    }
}
impl<R: BufRead> IRaxReader for RaxReader<R> {
    fn read_line(&mut self) -> miette::Result<Option<String>> {
        let mut buf = String::new();
        self.buf.clear();
        let n = self.inner.read_line(&mut buf).into_diagnostic()?;
        if n == 0 { Ok(None) } else { Ok(Some(buf)) }
    }

    fn read_lines_by_count(&mut self, count: usize) -> miette::Result<Vec<String>> {
        let mut lines = Vec::with_capacity(count);
        for _ in 0..count {
            match self.read_line()? {
                Some(line) => lines.push(line),
                None => break,
            }
        }
        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_read_line_some() {
        let data = "hello\nworld\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let line1 = reader.read_line().unwrap();
        assert_eq!(line1, Some("hello\n".to_string()));
        let line2 = reader.read_line().unwrap();
        assert_eq!(line2, Some("world\n".to_string()));
        let line3 = reader.read_line().unwrap();
        assert_eq!(line3, None);
    }

    #[test]
    fn test_read_lines_by_count_less_than_available() {
        let data = "a\nb\nc\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(2).unwrap();
        assert_eq!(lines, vec!["a\n".to_string(), "b\n".to_string()]);
    }

    #[test]
    fn test_read_lines_by_count_more_than_available() {
        let data = "x\ny\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(5).unwrap();
        assert_eq!(lines, vec!["x\n".to_string(), "y\n".to_string()]);
    }

    #[test]
    fn test_read_lines_by_count_zero() {
        let data = "foo\nbar\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(0).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_line_empty_input() {
        let data = "";
        let mut reader = RaxReader::new(Cursor::new(data));
        let line = reader.read_line().unwrap();
        assert_eq!(line, None);
    }

    #[test]
    fn test_read_lines_by_count_empty_input() {
        let data = "";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(3).unwrap();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_read_line_single_line_no_newline() {
        let data = "singleline";
        let mut reader = RaxReader::new(Cursor::new(data));
        let line = reader.read_line().unwrap();
        assert_eq!(line, Some("singleline".to_string()));
        let line2 = reader.read_line().unwrap();
        assert_eq!(line2, None);
    }

    #[test]
    fn test_read_lines_by_count_exact() {
        let data = "1\n2\n3\n";
        let mut reader = RaxReader::new(Cursor::new(data));
        let lines = reader.read_lines_by_count(3).unwrap();
        assert_eq!(
            lines,
            vec!["1\n".to_string(), "2\n".to_string(), "3\n".to_string()]
        );
    }
}
