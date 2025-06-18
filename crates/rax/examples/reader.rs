use std::io::BufReader;
use std::time::Duration;

use miette::IntoDiagnostic;
use rax::io::IRaxReader;

fn main() -> miette::Result<()> {
    let path = "COM4";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax::io::RaxReader::new(BufReader::new(port));
    loop {
        let message = reader.read_line()?;
        match message {
            Some(m) => println!("{}", m),
            None => (),
        }
    }
}
