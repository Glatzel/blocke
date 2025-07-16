use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Paragraph};

use crate::tab::NullCtx;
#[derive(Default)]
pub struct TabSettings {}
impl super::ITab<&NullCtx> for TabSettings {
    fn handle_key(&mut self, _key: KeyCode) {}
    fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect, _ctx: &NullCtx) {
        let p = Paragraph::new("Settings go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
            .block(Block::default().title("Settings"));
        f.render_widget(p, area);
    }
}
