use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Paragraph};

pub struct TabNmea {}
impl Default for TabNmea {
    fn default() -> Self { Self {} }
}
impl super::ITab for TabNmea {
    fn handle_key(&mut self, _key: KeyCode) { todo!() }

    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let p = Paragraph::new("NMEA go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
            .block(Block::default().title("NMEA"));
        f.render_widget(p, area);
    }
}
