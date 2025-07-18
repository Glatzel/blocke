use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use proj::Context;
use pyxis::crypto;
use rax::str_parser::StrParserContext;
use rax_nmea::data::{Gga, INmeaData, Identifier, Talker};

use crate::settings::SETTINGS;

pub struct TabCoord {
    parser: StrParserContext,
    _proj_context: Context,
}
impl Default for TabCoord {
    fn default() -> Self {
        let ctx = Context::default();
        let level = match SETTINGS.get().unwrap().verbose {
            clerk::LogLevel::ERROR => proj::LogLevel::Error,
            clerk::LogLevel::WARN => proj::LogLevel::Debug,
            clerk::LogLevel::INFO => proj::LogLevel::Debug,
            clerk::LogLevel::DEBUG => proj::LogLevel::Debug,
            clerk::LogLevel::TRACE => proj::LogLevel::Trace,
            clerk::LogLevel::OFF => proj::LogLevel::None,
        };
        ctx.set_log_level(level)
            .expect("Error to set proj log level.");
        Self {
            parser: StrParserContext::default(),
            _proj_context: ctx,
        }
    }
}
impl super::ITab for TabCoord {
    fn handle_key(&mut self, _key: KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        _f: &mut ratatui::Frame,
        _area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) {
        let gga = raw_nmea
            .iter()
            .rev()
            .find(|f| f.1 == Identifier::GGA)
            .map(|f| Gga::new(self.parser.init(f.2.to_string()), f.0).ok())
            .flatten();
        if let Some(gga) = gga {
            if let (Some(wgs84_lon), Some(wgs84_lat)) = (gga.lon(), gga.lat()) {
                let (wgs84_lon, wgs84_lat) = (wgs84_lon.clone(), wgs84_lat.clone());
                let (cgj02_lon, gcj02_lat) = crypto::wgs84_to_gcj02(wgs84_lon, wgs84_lat);
                let (_bd09_lon, _bd09_lat) = crypto::gcj02_to_bd09(cgj02_lon, gcj02_lat);
            }
        }
    }
    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
