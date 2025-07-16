use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Paragraph};

pub struct TabSettings {}
impl Default for TabSettings {
    fn default() -> Self { Self {} }
}
impl super::ITab for TabSettings {
    fn handle_key(&mut self, _key: KeyCode) { todo!() }

    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let p = Paragraph::new("Settings go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
            .block(Block::default().title("Settings"));
        f.render_widget(p, area);
    }
}
