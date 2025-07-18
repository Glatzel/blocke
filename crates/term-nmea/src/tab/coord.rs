use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use proj::Context;
use pyxis::crypto;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{Gga, INmeaData, Identifier, Talker};

use crate::settings::SETTINGS;

pub struct TabCoord {
    parser: StrParserContext,
    proj_context: Context,
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
            proj_context: ctx,
        }
    }
}
impl TabCoord {
    fn draw_table(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
        to_crs: &str,
    ) -> miette::Result<()> {
        let gga = raw_nmea
            .iter()
            .rev()
            .find(|f| f.1 == Identifier::GGA)
            .and_then(|f| Gga::new(self.parser.init(f.2.to_string()), f.0).ok());
        if let Some(gga) = gga {
            if let (Some(wgs84_lon), Some(wgs84_lat)) = (gga.lon(), gga.lat()) {
                let (wgs84_lon, wgs84_lat) = (*wgs84_lon, *wgs84_lat);
                let (cgj02_lon, gcj02_lat) = crypto::wgs84_to_gcj02(wgs84_lon, wgs84_lat);
                let (bd09_lon, bd09_lat) = crypto::gcj02_to_bd09(cgj02_lon, gcj02_lat);

                let (projected_x, projected_y) = match self.proj_context.create_crs_to_crs(
                    "EPSG:4326",
                    to_crs,
                    &proj::Area::default(),
                ) {
                    Ok(pj) => self
                        .proj_context
                        .normalize_for_visualization(&pj)?
                        .convert(&(wgs84_lon, wgs84_lat))?,
                    Err(e) => {
                        clerk::info!("{e}");
                        (f64::NAN, f64::NAN)
                    }
                };
                // Prepare rows: label and value pairs
                let rows = [
                    ("WGS84", wgs84_lon, wgs84_lat),
                    ("GCJ02", cgj02_lon, gcj02_lat),
                    ("BD09", bd09_lon, bd09_lat),
                    ("Projected", projected_x, projected_y),
                ];

                // Build Table rows for ratatui
                let table_rows = rows.iter().enumerate().map(|(i, r)| {
                    let bg = if i % 2 == 0 {
                        Color::White
                    } else {
                        Color::Gray
                    };
                    Row::new(vec![
                        Cell::from(r.0),
                        Cell::from(r.1.to_string()),
                        Cell::from(r.2.to_string()),
                    ])
                    .style(Style::default().bg(bg))
                });

                let table = Table::new(
                    table_rows,
                    &[
                        Constraint::Length(10),
                        Constraint::Percentage(45),
                        Constraint::Percentage(45),
                    ],
                )
                .header(
                    Row::new(vec!["CS", "X | Longitude", "Y | Latitude"]).style(
                        Style::default()
                            .fg(Color::Green)
                            .bg(Color::Gray)
                            .add_modifier(Modifier::BOLD),
                    ),
                )
                .column_spacing(2);
                f.render_widget(table, area);
            }
        }
        Ok(())
    }
    fn draw_projected_cs(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        to_crs: &str,
    ) -> miette::Result<()> {
        let input = Paragraph::new(to_crs)
            .block(Block::default().title("Projected CS").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow))
            .wrap(Wrap { trim: true });
        f.render_widget(input, area);

        Ok(())
    }
}
impl super::ITab for TabCoord {
    fn handle_key(&mut self, _key: KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) -> miette::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(1),
                Constraint::Min(4),
            ])
            .split(area);
        let to_crs = &SETTINGS.get().unwrap().tab_coord.projected_cs;
        self.draw_table(f, chunks[0], raw_nmea, to_crs)?;
        self.draw_projected_cs(f, chunks[2], to_crs)?;
        Ok(())
    }
    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
