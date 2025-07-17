use std::io::BufReader;
use std::time::Duration;

use clerk::tracing::level_filters::LevelFilter;
use miette::IntoDiagnostic;
use rax::io::IRaxReader;
fn main() -> miette::Result<()> {
    clerk::init_log_with_level(LevelFilter::TRACE);
    let path = "COM5";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax::io::RaxReader::new(BufReader::new(port));
    loop {
        if let Some(m) = reader.read_line()? {
            println!("{m}")
        }
    }
}
