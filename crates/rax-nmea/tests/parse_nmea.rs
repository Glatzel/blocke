use std::fs::File;
use std::io;

use clerk::init_log_with_level;
use clerk::tracing::level_filters::LevelFilter;
use miette::IntoDiagnostic;
use rax_nmea::Dispatcher;
use rax_nmea::data::{
    Dhv, Gbs, Gga, Gll, Gns, Grs, Gst, Gsv, INmeaData, Identifier, Rmc, Txt, Vtg, Zda,
};
use rax::io::RaxReader;
use rax::str_parser::StrParserContext;
#[test]
fn test_parse_nmea() -> miette::Result<()> {
    init_log_with_level(LevelFilter::WARN);
    for f in [
        "data/nmea1.log",
        "data/nmea2.log",
        "data/nmea_with_sat_info.log",
    ] {
        let mut reader = RaxReader::new(io::BufReader::new(File::open(f).into_diagnostic()?));
        let mut ctx = StrParserContext::new();
        let dispatcher = Dispatcher::new(&mut reader);

        for (talker, identifier, sentence) in dispatcher {
            match identifier {
                Identifier::DHV => {
                    let ctx = ctx.init(sentence);
                    let _ = Dhv::new(ctx, talker)?;
                }
                Identifier::GBS => {
                    let ctx = ctx.init(sentence);
                    let _ = Gbs::new(ctx, talker)?;
                }
                Identifier::GGA => {
                    let ctx = ctx.init(sentence);
                    let _ = Gga::new(ctx, talker)?;
                }
                Identifier::GLL => {
                    let ctx = ctx.init(sentence);
                    let _ = Gll::new(ctx, talker)?;
                }
                Identifier::GNS => {
                    let ctx = ctx.init(sentence);
                    let _ = Gns::new(ctx, talker)?;
                }
                Identifier::GRS => {
                    let ctx = ctx.init(sentence);
                    let _ = Gbs::new(ctx, talker)?;
                }
                Identifier::GSA => {
                    let ctx = ctx.init(sentence);
                    let _ = Grs::new(ctx, talker)?;
                }
                Identifier::GST => {
                    let ctx = ctx.init(sentence);
                    let _ = Gst::new(ctx, talker)?;
                }
                Identifier::GSV => {
                    let ctx = ctx.init(sentence);
                    let _ = Gsv::new(ctx, talker)?;
                }
                Identifier::RMC => {
                    let ctx = ctx.init(sentence);
                    let _ = Rmc::new(ctx, talker)?;
                }
                Identifier::Txt => {
                    let ctx = ctx.init(sentence);
                    let _ = Txt::new(ctx, talker)?;
                }
                Identifier::VTG => {
                    let ctx = ctx.init(sentence);
                    let _ = Vtg::new(ctx, talker)?;
                }
                Identifier::ZDA => {
                    let ctx = ctx.init(sentence);
                    let _ = Zda::new(ctx, talker)?;
                }
            }
        }
    }
    Ok(())
}
