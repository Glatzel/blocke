use std::io::BufReader;
use std::str::FromStr;
use std::time::Duration;

use miette::IntoDiagnostic;
use rax::io::IRaxReader;
use rax_nmea::nmea_data::{Dhv, Gga, Gll, Gsa, NavigationSystem, NmeaDataType, Vtg, Zda};
fn main() -> miette::Result<()> {
    let path = "COM4";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax::io::RaxReader::new(BufReader::new(port));

    loop {
        let message = reader.read_line()?;
        if let Some(m) = message {
            // Process the message in a new scope so the borrow ends before the next iteration
            {
                match NmeaDataType::from_str(m) {
                    Ok(nv) => match nv {
                        NmeaDataType::DHV => {
                            // let nmea = Dhv::new(m,
                            // NavigationSystem::from_str(m)?);
                            // println!("{:?}", nmea)
                        }
                        NmeaDataType::GGA => {
                            // let nmea = Gga::new(m,
                            // NavigationSystem::from_str(m)?);
                            // println!("{:?}", nmea)
                        }
                        NmeaDataType::GLL => {
                            // let nmea = Gll::new(m, NavigationSystem::from_str(m)?);
                            // println!("{:?}", nmea)
                        }
                        NmeaDataType::GSA => {
                            // let nmea = Gsa::new(m,
                            // NavigationSystem::from_str(m)?);
                            // println!("{:?}", nmea)
                        }
                        NmeaDataType::VTG => {
                            // let nmea = Vtg::new(m,
                            // NavigationSystem::from_str(m)?);
                            // println!("{:?}", nmea)
                        }
                        NmeaDataType::ZDA => {
                            // let nmea = Zda::new(m,
                            // NavigationSystem::from_str(m)?);
                            // println!("{:?}", nmea)
                        }
                        NmeaDataType::Other(_) => (),
                    },
                    Err(_) => (),
                }
            }
        }
    }
}
