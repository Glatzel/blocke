use std::fs::File;
use std::io;

use miette::IntoDiagnostic;
use rax_nmea::Dispatcher;
use rax_nmea::data::{Dhv, Gga, Gll, Gsa, Gst, Gsv, INmeaData, Identifier, Rmc, Txt, Vtg, Zda};
use rax_parser::io::RaxReader;
use rax_parser::str_parser::StrParserContext;
use test_utils::init_log;
#[test]
fn test_parse() -> miette::Result<()> {
    init_log();
    for f in [
        "data/nmea1.log",
        // "data/nmea2.log",
        // "data/nmea_with_sat_info.log",
    ] {
        let mut reader = RaxReader::new(io::BufReader::new(File::open(f).into_diagnostic()?));
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
    }

    Ok(())
}
