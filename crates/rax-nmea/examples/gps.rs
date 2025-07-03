use std::io::BufReader;
use std::time::Duration;

use clerk::tracing::level_filters::LevelFilter;
use miette::IntoDiagnostic;
use rax_nmea::Dispatcher;
use rax_nmea::data::{Dhv, Gga, Gll, Gsa, Gst, Gsv, INmeaData, Identifier, Rmc, Txt, Vtg, Zda};
use rax_parser::str_parser::StrParserContext;
fn main() -> miette::Result<()> {
    test_utils::init_log_with_level(LevelFilter::WARN);
    let path = "COM3";
    let port = serialport::new(path, 9600)
        .timeout(Duration::from_millis(3000))
        .open()
        .into_diagnostic()?;
    let mut reader = rax_parser::io::RaxReader::new(BufReader::new(port));
    let mut ctx = StrParserContext::new();
    let dispatcher = Dispatcher::new(&mut reader);
    for (talker, identifier, sentence) in dispatcher {
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
            Identifier::GSV => {
                let ctx = ctx.init(sentence);
                let nmea = Gsv::new(ctx, talker)?;
                println!("{:?}", nmea)
            }
            Identifier::RMC => {
                let ctx = ctx.init(sentence);
                let nmea = Rmc::new(ctx, talker)?;
                println!("{:?}", nmea)
            }
            Identifier::Txt => {
                let ctx = ctx.init(sentence);
                let nmea = Txt::new(ctx, talker)?;
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
    Ok(())
}
