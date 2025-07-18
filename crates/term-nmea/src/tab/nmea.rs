use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use rax_nmea::data::{Identifier, Talker};

#[derive(Default)]
pub struct TabNmea {}

impl super::ITab for TabNmea {
    fn handle_key(&mut self, _key: KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    )-> miette::Result<()> {
        let count = raw_nmea.len();

        let visible_lines = area.height as usize;

        let p = Paragraph::new(
            raw_nmea
                .iter()
                .skip(count.saturating_sub(visible_lines))
                .map(|f| Line::from(f.2.as_str()))
                .collect::<Vec<Line>>(),
        );

        f.render_widget(p, area);
        Ok(())
    }

    fn hint(&mut self) -> &'static [&'static str] { &["`b` Lock to Bottom", "`↑↓` Scroll"] }
}
