use ratatui::widgets::{Block, Paragraph};

use crate::tab::NullCtx;

#[derive(Default)]
pub struct TabInfo {}
impl super::ITab<&NullCtx> for TabInfo {
    fn handle_key(&mut self, _key: crossterm::event::KeyCode) {}

    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, _ctx: &NullCtx) {
        let p = Paragraph::new("Info go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
            .block(Block::default().title("Info"));
        f.render_widget(p, area);
    }
}
