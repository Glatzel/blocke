use std::collections::VecDeque;

use crossterm::event::KeyCode;
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, ScrollbarState};

#[derive(Default)]
pub struct TabNmea {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}
impl super::ITab<&VecDeque<std::string::String>> for TabNmea {
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Down => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            KeyCode::Up => {
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            KeyCode::Left => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
                self.horizontal_scroll_state = self
                    .horizontal_scroll_state
                    .position(self.horizontal_scroll);
            }
            KeyCode::Right => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
                self.horizontal_scroll_state = self
                    .horizontal_scroll_state
                    .position(self.horizontal_scroll);
            }
            _ => {}
        }
    }

    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        ctx: &VecDeque<std::string::String>,
    ) {
        let p = Paragraph::new(
            ctx.iter()
                .map(|l| Line::from(l.as_str()))
                .collect::<Vec<Line>>(),
        )
        .scroll((self.vertical_scroll as u16, 0));
        f.render_widget(p, area);
    }
}
