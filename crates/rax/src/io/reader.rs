use std::io::BufRead;

use miette::IntoDiagnostic;
pub trait IRaxReader {
    /// Reads the next line; returns `None` on EOF.
    fn read_line(&mut self) -> miette::Result<Option<&str>>;
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
    fn read_line(&mut self) -> miette::Result<Option<&str>> {
        self.buf.clear();
        let n = self.inner.read_line(&mut self.buf).into_diagnostic()?;
        if n == 0 {
            Ok(None)
        } else {
            Ok(Some(self.buf.trim_end()))
        }
    }
}
