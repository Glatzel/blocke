use async_trait::async_trait;
use miette::IntoDiagnostic;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// Async counterpart of `IRaxReader`.
#[async_trait]
pub trait AsyncIRaxReader {
    async fn read_line(&mut self) -> miette::Result<Option<String>>;
    async fn read_lines_by_count(&mut self, count: usize) -> miette::Result<Vec<String>>;
}

/// Buffered async reader implementing `AsyncIRaxReader`.
pub struct AsyncRaxReader<R: AsyncBufRead + Unpin> {
    inner: R,
}

impl<R: AsyncBufRead + Unpin> AsyncRaxReader<R> {
    pub fn new(inner: R) -> Self { Self { inner } }
}

#[async_trait]
impl<R> AsyncIRaxReader for AsyncRaxReader<R>
where
    R: AsyncBufRead + Unpin + Send, // `Send` lets it cross await points safely
{
    async fn read_line(&mut self) -> miette::Result<Option<String>> {
        let mut buf = String::new();
        let n = self.inner.read_line(&mut buf).await.into_diagnostic()?;
        clerk::debug!(
            "[AsyncRaxReader] read_line: bytes read = {}, line = {:?}",
            n,
            buf
        );
        Ok((n > 0).then_some(buf))
    }

    async fn read_lines_by_count(&mut self, count: usize) -> miette::Result<Vec<String>> {
        let mut lines = Vec::with_capacity(count);
        for i in 0..count {
            match self.read_line().await? {
                Some(line) => {
                    clerk::debug!(
                        "[AsyncRaxReader] read_lines_by_count: line {} = {:?}",
                        i + 1,
                        line
                    );
                    lines.push(line);
                }
                None => break,
            }
        }
        Ok(lines)
    }
}
