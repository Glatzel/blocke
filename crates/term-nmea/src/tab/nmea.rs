use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEventKind};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, ScrollbarState};

#[derive(Default)]
pub struct TabNmea {
    pub raw_nmea: VecDeque<String>,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}
impl super::ITab for TabNmea {
    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
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
    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            MouseEventKind::ScrollUp => {
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            MouseEventKind::ScrollLeft => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
                self.horizontal_scroll_state = self
                    .horizontal_scroll_state
                    .position(self.horizontal_scroll);
            }
            MouseEventKind::ScrollRight => {
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
        _raw_nmea: &VecDeque<String>,
    ) {
        let p = Paragraph::new(
            self.raw_nmea
                .iter()
                .map(|f| Line::from(f.as_str()))
                .collect::<Vec<Line>>(),
        )
        .scroll((self.vertical_scroll as u16, 0));
        f.render_widget(p, area);
    }
}
