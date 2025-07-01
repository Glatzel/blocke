use std::io::BufReader;
use std::time::Duration;

use miette::IntoDiagnostic;
use rax_nmea::Dispatcher;
use rax_nmea::data::{Dhv, Gga, Gll, Gsa, Gst, INmeaData, Identifier, Rmc, Vtg, Zda};
use rax_parser::str_parser::StrParserContext;
fn main() -> miette::Result<()> {
    test_utils::init_log();
    #[cfg(target_os = "windows")]
    let path = "COM3";
    #[cfg(target_os = "linux")]
    let path = "/dev/ttyUSB0";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax_parser::io::RaxReader::new(BufReader::new(port));
    let mut ctx = StrParserContext::new();
    let mut dispatcher = Dispatcher::new(&mut reader);
    loop {
        let current = dispatcher.next();
        if let Some((talker, identifier, sentence)) = current {
            match identifier {
                Identifier::DHV => {
                    let ctx = ctx.init(sentence);
                    let nmea = Dhv::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::GGA => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gga::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::GLL => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gll::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::GSA => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gsa::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::GST => {
                    let ctx = ctx.init(sentence);
                    let nmea = Gst::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::GSV => println!("\nGSV#######################\n{sentence}######################\n"),
                Identifier::RMC => {
                    let ctx = ctx.init(sentence);
                    let nmea = Rmc::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::VTG => {
                    let ctx = ctx.init(sentence);
                    let nmea = Vtg::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
                Identifier::ZDA => {
                    let ctx = ctx.init(sentence);
                    let nmea = Zda::new(ctx, talker)?;
                    println!("{:?}", nmea)
                }
            }
        }
    }
}
