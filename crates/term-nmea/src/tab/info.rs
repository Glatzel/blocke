use std::collections::VecDeque;

use ratatui::widgets::{Block, Paragraph};

#[derive(Default)]
pub struct TabInfo {}
impl super::ITab for TabInfo {
    fn handle_key(&mut self, _key: crossterm::event::KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _raw_nmea: &VecDeque<String>,
    ) {
        let p = Paragraph::new("Todo!").block(Block::default().title("Info"));
        f.render_widget(p, area);
    }

    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
