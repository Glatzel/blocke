use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Paragraph};

#[derive(Default)]
pub struct TabSettings {}
impl super::ITab for TabSettings {
    fn handle_key(&mut self, _key: KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _raw_nmea: &VecDeque<String>,
    ) {
        let _ = _raw_nmea;
        let p = Paragraph::new("Todo!").block(Block::default().title("Settings"));
        f.render_widget(p, area);
    }

    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
