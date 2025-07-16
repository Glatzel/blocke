use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use tokio_serial::SerialPortBuilderExt;

pub async fn start_serial_reader(
    port: String,
    baud_rate: u32,
    tx: Sender<String>,
) -> miette::Result<()> {
    let serial = match tokio_serial::new(port, baud_rate).open_native_async() {
        Ok(s) => s,
        Err(e) => {
            miette::bail!("Failed to open serial port: {}", e);
        }
    };

    let reader = BufReader::new(serial);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let _ = tx.send(line).await;
    }
    Ok(())
}
