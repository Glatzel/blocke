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
    loop {
        let message = reader.read_line()?;
        if let Some(m) = message {
            // Process the message in a new scope so the borrow ends before the next
            // iteration
            {
                if let Ok(t) = Identifier::from_str(&m) {
                    if let Ok(nv) = Talker::from_str(&m) {
                        match t {
                            Identifier::DHV => {
                                let ctx = ctx.init(m);
                                let nmea = Dhv::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::GGA => {
                                let ctx = ctx.init(m);
                                let nmea = Gga::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::GLL => {
                                let ctx = ctx.init(m);
                                let nmea = Gll::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::GSA => {
                                let ctx = ctx.init(m);
                                let nmea = Gsa::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::GST => {
                                let ctx = ctx.init(m);
                                let nmea = Gst::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::RMC => {
                                let ctx = ctx.init(m);
                                let nmea = Rmc::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::VTG => {
                                let ctx = ctx.init(m);
                                let nmea = Vtg::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                            Identifier::ZDA => {
                                let ctx = ctx.init(m);
                                let nmea = Zda::new(ctx, nv)?;
                                println!("{:?}", nmea)
                            }
                        }
                    }
                }
            }
        }
    }
}
