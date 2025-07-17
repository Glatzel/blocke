use std::collections::VecDeque;

use ratatui::layout::Constraint;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Cell, Row, Table};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{Gga, Gsa, Gst, INmeaData, Identifier, Rmc, Talker};

#[derive(Default)]
pub struct TabInfo {
    ctx: StrParserContext,
}
impl super::ITab for TabInfo {
    fn handle_key(&mut self, _key: crossterm::event::KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) {
        // Get last sentences
        let (gga, rmc, gsa, gst) = Self::find_last_sentence(&mut self.ctx, raw_nmea);
        let lon = gga
            .as_ref()
            .and_then(|f| f.lon().map(|v| v.to_string()))
            .unwrap_or_default();
        let lat = gga
            .as_ref()
            .and_then(|f| f.lat().map(|v| v.to_string()))
            .unwrap_or_default();
        let alt = gga
            .as_ref()
            .and_then(|f| f.alt().map(|v| v.to_string()))
            .unwrap_or_default();
        let time = gga
            .as_ref()
            .and_then(|f| f.time().map(|v| v.to_string()))
            .unwrap_or_default();
        let speed = rmc
            .as_ref()
            .and_then(|f| f.spd().map(|v| v.to_string()))
            .unwrap_or_default();
        let heading = rmc
            .as_ref()
            .as_ref()
            .and_then(|f| f.cog().map(|v| v.to_string()))
            .unwrap_or_default();
        let quality = gga
            .as_ref()
            .and_then(|f| f.quality().map(|v| v.to_string()))
            .unwrap_or_default();
        let pos_mode = rmc
            .as_ref()
            .and_then(|f| f.pos_mode().map(|v| v.to_string()))
            .unwrap_or_default();
        let pdop = gsa
            .as_ref()
            .and_then(|f| f.pdop().map(|v| v.to_string()))
            .unwrap_or_default();
        let hdop = gsa
            .as_ref()
            .and_then(|f| f.hdop().map(|v| v.to_string()))
            .unwrap_or_default();
        let vdop = gsa
            .as_ref()
            .and_then(|f| f.vdop().map(|v| v.to_string()))
            .unwrap_or_default();
        let rms = gst
            .as_ref()
            .and_then(|f| f.rms().map(|v| v.to_string()))
            .unwrap_or_default();
        let std_lon = gst
            .as_ref()
            .and_then(|f| f.std_lon().map(|v| v.to_string()))
            .unwrap_or_default();
        let std_lat = gst
            .as_ref()
            .as_ref()
            .and_then(|f| f.std_lat().map(|v| v.to_string()))
            .unwrap_or_default();
        let std_alt = gst
            .as_ref()
            .and_then(|f| f.std_alt().map(|v| v.to_string()))
            .unwrap_or_default();
        // Prepare rows: label and value pairs
        let rows = vec![
            vec!["Longitude", &lon],
            vec!["Latitude", &lat],
            vec!["Altitude", &alt],
            vec!["Time (UTC)", &time],
            vec!["Speed (knots)", &speed],
            vec!["Heading (°)", &heading],
            vec!["Position Quality", &quality],
            vec!["Position Mode", &pos_mode],
            vec!["PDOP", &pdop],
            vec!["HDOP", &hdop],
            vec!["VDOP", &vdop],
            vec!["Pseudorange RMS", &rms],
            vec!["Lon Std Dev", &std_lon],
            vec!["Lat Std Dev", &std_lat],
            vec!["Alt Std Dev", &std_alt],
        ];

        // Build Table rows for ratatui
        let table_rows = rows.iter().enumerate().map(|(i, r)| {
            let bg = if i % 2 == 0 {
                Color::White
            } else {
                Color::Gray
            };
            Row::new(vec![Cell::from(r[0]), Cell::from(r[1])]).style(Style::default().bg(bg))
        });

        let table = Table::new(table_rows, &[Constraint::Length(20), Constraint::Min(10)])
            .header(
                Row::new(vec!["Field", "Value"]).style(
                    Style::default()
                        .fg(Color::Green)
                        .bg(Color::Gray)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .column_spacing(2);

        f.render_widget(table, area);
    }
    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
impl TabInfo {
    fn find_last_sentence(
        ctx: &mut StrParserContext,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) -> (
        Option<Gga>, // GGA
        Option<Rmc>, // RMC
        Option<Gsa>, // GSA
        Option<Gst>, // GST
    ) {
        let mut last_gga = None;
        let mut last_rmc = None;
        let mut last_gsa = None;
        let mut last_gst = None;

        for item in raw_nmea.iter().rev() {
            match item.1 {
                Identifier::GGA if last_gga.is_none() => last_gga = Some(item.clone()),
                Identifier::RMC if last_rmc.is_none() => last_rmc = Some(item.clone()),
                Identifier::GSA if last_gsa.is_none() => last_gsa = Some(item.clone()),
                Identifier::GST if last_gst.is_none() => last_gst = Some(item.clone()),
                _ => {}
            }

            if last_gga.is_some() && last_rmc.is_some() && last_gsa.is_some() && last_gst.is_some()
            {
                break; // found all, early exit
            }
        }
        let last_gga = last_gga
            .and_then(|(talker, _identityer, sentence)| Gga::new(ctx.init(sentence), talker).ok());
        let last_rmc = last_rmc
            .and_then(|(talker, _identityer, sentence)| Rmc::new(ctx.init(sentence), talker).ok());
        let last_gsa = last_gsa
            .and_then(|(talker, _identityer, sentence)| Gsa::new(ctx.init(sentence), talker).ok());
        let last_gst = last_gst
            .and_then(|(talker, _identityer, sentence)| Gst::new(ctx.init(sentence), talker).ok());
        (last_gga, last_rmc, last_gsa, last_gst)
    }
}
