use std::io::BufReader;
use std::time::Duration;

use miette::IntoDiagnostic;
use rax_parser::io::IRaxReader;
fn main() -> miette::Result<()> {
    test_utils::init_log();
    #[cfg(target_os = "windows")]
    let path = "COM4";
    #[cfg(target_os = "linux")]
    let path = "/dev/ttyUSB0";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax_parser::io::RaxReader::new(BufReader::new(port));
    loop {
        let message = reader.read_line()?;
        if let Some(m) = message {
            println!("{}", m)
        }
    }
}
