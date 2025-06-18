use std::io::{self, BufRead, BufReader};
use std::sync::LazyLock;
use std::time::Duration;

use nmea::INmeaData;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT_LOGGING: LazyLock<bool> = LazyLock::new(|| {
    tracing_subscriber::registry()
        .with(clerk::terminal_layer(LevelFilter::TRACE, true))
        .init();
    true
});
pub fn init_log() { let _ = &*INIT_LOGGING; }

fn main() -> io::Result<()> {
    let port_name = "COM3"; // or COM3 on Windows
    let baud_rate = 9600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    // Wrap in a BufReader for line-based reading
    let mut reader = BufReader::new(port);
    // let ctx = nmea::NmeaContext::new(10, Some(&PathBuf::from("a.txt")), None,
    // None).unwrap();
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            // Timeout reached or device disconnected
            continue;
        }
        // println!("Received line: {}", line.trim_end());

        match nmea::nmea_data::Gll::parse_sentence(
            line.trim_end(),
            nmea::nmea_data::NavigationSystem::BD,
        ) {
            Ok(info) => println!(
                "longitude: {}, latitude: {}",
                info.lon().unwrap(),
                info.lat().unwrap()
            ),
            Err(_) => (),
        };
    }
}
