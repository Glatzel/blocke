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
