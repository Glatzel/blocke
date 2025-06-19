use std::io::BufReader;
use std::str::FromStr;
use std::time::Duration;

use miette::IntoDiagnostic;
use rax::io::IRaxReader;
use rax::str_parser::StrParserContext;
use rax_nmea::nmea_data::{
    Dhv, Gga, Gll, Gsa, INmeaData, NavigationSystem, NmeaDataType, Vtg, Zda,
};
fn main() -> miette::Result<()> {
    let path = "COM4";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax::io::RaxReader::new(BufReader::new(port));
    let mut ctx = StrParserContext::new();
    loop {
        let message = reader.read_line()?;
        if let Some(m) = message {
            // Process the message in a new scope so the borrow ends before the next
            // iteration
            {
                let nv = NavigationSystem::from_str(&m)?;
                match NmeaDataType::from_str(&m) {
                    Ok(t) => match t {
                        NmeaDataType::DHV => {
                            let ctx = ctx.init(m);
                            let nmea = Dhv::new(ctx, nv)?;
                            println!("{:?}", nmea)
                        }
                        NmeaDataType::GGA => {
                            let ctx = ctx.init(m);
                            let nmea = Gga::new(ctx, nv)?;
                            println!("{:?}", nmea)
                        }
                        NmeaDataType::GLL => {
                            let ctx = ctx.init(m);
                            let nmea = Gll::new(ctx, nv)?;
                            println!("{:?}", nmea)
                        }
                        NmeaDataType::GSA => {
                            let ctx = ctx.init(m);
                            let nmea = Gsa::new(ctx, nv)?;
                            println!("{:?}", nmea)
                        }
                        NmeaDataType::VTG => {
                            let ctx = ctx.init(m);
                            let nmea = Vtg::new(ctx, nv)?;
                            println!("{:?}", nmea)
                        }
                        NmeaDataType::ZDA => {
                            let ctx = ctx.init(m);
                            let nmea = Zda::new(ctx, nv)?;
                            println!("{:?}", nmea)
                        }
                        NmeaDataType::Other(_) => (),
                    },
                    Err(_) => (),
                }
            }
        }
    }
}
