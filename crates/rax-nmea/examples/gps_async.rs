use clerk::tracing::level_filters::LevelFilter;
use futures::StreamExt;
use miette::IntoDiagnostic;
use rax::io::AsyncRaxReader;
use rax::str_parser::StrParserContext;
use rax_nmea::AsyncDispatcher;
use rax_nmea::data::*;
use tokio::io::BufReader;
use tokio_serial::SerialPortBuilderExt;
#[tokio::main]
async fn main() -> miette::Result<()> {
    clerk::init_log_with_level(LevelFilter::WARN);
    let port = "COM5";
    let serial = tokio_serial::new(port, 9600)
        .open_native_async()
        .into_diagnostic()?;
    let mut reader = AsyncRaxReader::new(BufReader::new(serial));
    let mut dispatcher = AsyncDispatcher::new(&mut reader);
    let mut ctx = StrParserContext::new();
    while let Some((talker, identifier, sentence)) = dispatcher.next().await {
        match identifier {
            Identifier::DHV => {
                let ctx = ctx.init(sentence);
                let nmea = Dhv::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GBS => {
                let ctx = ctx.init(sentence);
                let nmea = Gbs::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GGA => {
                let ctx = ctx.init(sentence);
                let nmea = Gga::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GLL => {
                let ctx = ctx.init(sentence);
                let nmea = Gll::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GNS => {
                let ctx = ctx.init(sentence);
                let nmea = Gns::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GRS => {
                let ctx = ctx.init(sentence);
                let nmea = Grs::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GSA => {
                let ctx = ctx.init(sentence);
                let nmea = Gsa::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GST => {
                let ctx = ctx.init(sentence);
                let nmea = Gst::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GSV => {
                let ctx = ctx.init(sentence);
                let nmea = Gsv::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::RMC => {
                let ctx = ctx.init(sentence);
                let nmea = Rmc::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::TXT => {
                let ctx = ctx.init(sentence);
                let nmea = Txt::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::VTG => {
                let ctx = ctx.init(sentence);
                let nmea = Vtg::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::ZDA => {
                let ctx = ctx.init(sentence);
                let nmea = Zda::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::DTM => {
                let ctx = ctx.init(sentence);
                let nmea = Dtm::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GBQ => {
                let ctx = ctx.init(sentence);
                let nmea = Gbq::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GLQ => {
                let ctx = ctx.init(sentence);
                let nmea = Glq::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GNQ => {
                let ctx = ctx.init(sentence);
                let nmea = Gnq::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::GPQ => {
                let ctx = ctx.init(sentence);
                let nmea = Gpq::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::THS => {
                let ctx = ctx.init(sentence);
                let nmea = Ths::new(ctx, talker)?;
                println!("{nmea:?}")
            }
            Identifier::VLW => {
                let ctx = ctx.init(sentence);
                let nmea = Vlw::new(ctx, talker)?;
                println!("{nmea:?}")
            }
        }
    }
    Ok(())
}
