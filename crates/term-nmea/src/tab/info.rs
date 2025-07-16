use ratatui::widgets::{Block, Paragraph, Widget};

pub struct TabInfo {}
impl Default for TabInfo {
    fn default() -> Self { Self {} }
}
impl super::ITab for TabInfo {
    fn handle_key(&mut self, _key: crossterm::event::KeyCode) { todo!() }

    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let p = Paragraph::new("Info go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
            .block(Block::default().title("Info"));
        f.render_widget(p, area);
    }
}
