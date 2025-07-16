use miette::IntoDiagnostic;
use rax::io::{AsyncIRaxReader, AsyncRaxReader};
use rax_nmea::Dispatcher;
use rax_nmea::data::{Identifier, Talker};
use tokio::io::BufReader;
use tokio::sync::mpsc::Sender;
use tokio_serial::SerialPortBuilderExt;

pub async fn start_serial_reader(
    port: String,
    baud_rate: u32,
    tx: Sender<(Talker, Identifier, String)>,
) -> miette::Result<()> {
    let serial = tokio_serial::new(port, baud_rate)
        .open_native_async()
        .into_diagnostic()?;
    let mut reader = AsyncRaxReader::new(BufReader::new(serial));
    let mut dispatcher = Dispatcher::new();
    while let Some(line) = reader.read_line().await? {
        if let Some((talker, identifier, sentence)) = dispatcher.dispatch(line) {
            let _ = tx.send((talker, identifier, sentence)).await;
        }
    }
    Ok(())
}
