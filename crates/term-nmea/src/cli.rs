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

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
#[cfg(test)]
mod tests {
    use clap_verbosity_flag::log;

    use super::*;

    #[test]
    fn test_default_cli_args() {
        let args = CliArgs::parse_from(["term-nmea"]);
        assert_eq!(args.port, None);
        assert_eq!(args.baud_rate, None);
        assert_eq!(args.capacity, None);
    }

    #[test]
    fn test_cli_parsing_port_and_baud_rate() {
        let args = CliArgs::parse_from(["term-nmea", "--port", "COM3", "--baud-rate", "115200"]);
        assert_eq!(args.port.as_deref(), Some("COM3"));
        assert_eq!(args.baud_rate, Some(115200));
    }

    #[test]
    fn test_cli_parsing_with_short_flags() {
        let args = CliArgs::parse_from(["term-nmea", "-p", "COM9", "-b", "38400", "-c", "512"]);
        assert_eq!(args.port.as_deref(), Some("COM9"));
        assert_eq!(args.baud_rate, Some(38400));
        assert_eq!(args.capacity, Some(512));
    }

    #[test]
    fn test_cli_parsing_verbosity() {
        let args = CliArgs::parse_from(["term-nmea", "-v"]);
        assert!(args.verbose.log_level().is_some());

        let args = CliArgs::parse_from(["term-nmea", "-vvv"]);
        assert_eq!(args.verbose.log_level().unwrap(), log::Level::Trace);
    }
}
