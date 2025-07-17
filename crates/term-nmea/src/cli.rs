use clap::Parser;

/// Command-line arguments that override `term-nmea.toml`.
#[derive(Debug, Parser)]
#[command(name = "term-nmea", version, about = "Terminal NMEA reader")]
pub struct CliArgs {
    /// Serial port to open
    #[arg(short, long)]
    pub port: Option<String>,

    /// Baud rate of the serial port
    #[arg(short, long)]
    pub baud_rate: Option<u32>,

    /// Line buffer capacity
    #[arg(short, long)]
    pub capacity: Option<usize>,
}
