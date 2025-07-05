use miette::{IntoDiagnostic, Result};
use rax::io::{AsyncIRaxReader, AsyncRaxReader};
use tokio::fs::File;
use tokio::io::BufReader; // async API

#[tokio::main]
async fn main() -> Result<()> {
    // Open the file asynchronously
    let file = File::open("input.txt").await.into_diagnostic()?;
    let buf_reader = BufReader::new(file);

    // Wrap it in AsyncRaxReader
    let mut reader = AsyncRaxReader::new(buf_reader);

    // Drain the file line‑by‑line
    while let Some(line) = reader.read_line().await? {
        print!("{}", line);
    }
    Ok(())
}
