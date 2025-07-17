use miette::{Context, IntoDiagnostic};
use rax::io::{AsyncIRaxReader, AsyncRaxReader};
use rax_nmea::Dispatcher;
use rax_nmea::data::{Identifier, Talker};
use tokio::io::BufReader;
use tokio::sync::mpsc::Sender;
use tokio_serial::SerialPortBuilderExt;
async fn run_serial_dispatcher<R: AsyncIRaxReader>(
    mut reader: R,
    tx: Sender<(Talker, Identifier, String)>,
) -> miette::Result<()> {
    let mut dispatcher = Dispatcher::new();
    while let Some(msg) = reader
        .read_line()
        .await?
        .and_then(|l| dispatcher.dispatch(l))
    {
        let _ = tx.send(msg).await;
    }
    Ok(())
}
pub async fn start_serial_reader(
    port: String,
    baud_rate: u32,
    tx: Sender<(Talker, Identifier, String)>,
) -> miette::Result<()> {
    if !tokio_serial::available_ports()
        .into_diagnostic()?
        .iter()
        .any(|p| p.port_name.eq_ignore_ascii_case(&port))
    {
        let msg = format!("Port '{port}' is not available");
        clerk::error!("{msg}");
        if !cfg!(debug_assertions) {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    }

    let serial = tokio_serial::new(port.clone(), baud_rate)
        .open_native_async()
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to open serial port: {port}"))?;
    let reader = AsyncRaxReader::new(BufReader::new(serial));
    run_serial_dispatcher(reader, tx).await
}
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use async_trait::async_trait;
    use rax::io::AsyncIRaxReader;
    use tokio::sync::mpsc;

    use super::*;

    struct MockRaxReader {
        lines: VecDeque<&'static str>,
    }

    #[async_trait]
    impl AsyncIRaxReader for MockRaxReader {
        async fn read_line(&mut self) -> miette::Result<Option<String>> {
            Ok(self.lines.pop_front().map(str::to_string))
        }
        async fn read_lines_by_count(&mut self, _count: usize) -> miette::Result<Vec<String>> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_run_serial_dispatcher() -> miette::Result<()> {
        let input = vec![
            "$GPGSA,A,3,04,05,..*1C\r\n", // GSA sentence
            "$GPGGA,1234,...*55\r\n",     // GGA sentence
        ];
        let reader = MockRaxReader {
            lines: input.into_iter().collect(),
        };

        let (tx, mut rx) = mpsc::channel(10);
        run_serial_dispatcher(reader, tx).await?;

        let mut received = vec![];
        while let Ok(msg) = rx.try_recv() {
            received.push(msg);
        }

        assert!(received.iter().any(|(_, id, _)| *id == Identifier::GSA));
        assert!(received.iter().any(|(_, id, _)| *id == Identifier::GGA));
        Ok(())
    }
}
