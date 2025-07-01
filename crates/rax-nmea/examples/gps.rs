use std::io::BufReader;
use std::str::FromStr;
use std::time::Duration;

use miette::IntoDiagnostic;
use rax_nmea::data::{Dhv, Gga, Gll, Gsa, Gst, INmeaData, Identifier, Rmc, Talker, Vtg, Zda};
use rax_parser::io::IRaxReader;
use rax_parser::str_parser::StrParserContext;
fn main() -> miette::Result<()> {
    // test_utils::init_log();
    #[cfg(target_os = "windows")]
    let path = "COM4";
    #[cfg(target_os = "linux")]
    let path = "/dev/ttyUSB0";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax_parser::io::RaxReader::new(BufReader::new(port));
    let mut ctx = StrParserContext::new();
    loop {}
}
