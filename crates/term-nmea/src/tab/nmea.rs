use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEventKind};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use rax_nmea::data::{Identifier, Talker};

pub struct TabNmea {
    pub lock_to_bottom: bool,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
}
impl Default for TabNmea {
    fn default() -> Self {
        Self {
            lock_to_bottom: true,
            vertical_scroll_state: Default::default(),
            vertical_scroll: Default::default(),
        }
    }
}
impl super::ITab for TabNmea {
    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Down => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
            }
            KeyCode::Up => {
                self.lock_to_bottom = false;
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
            }
            KeyCode::Char('b') => self.lock_to_bottom = true,
            _ => {}
        }
    }
    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
            }
            MouseEventKind::ScrollUp => {
                self.lock_to_bottom = false;
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
            }
            _ => {}
        }
    }
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) {
        let count = raw_nmea.len();

        let visible_lines = area.height as usize;
        if self.vertical_scroll + visible_lines >= count || self.lock_to_bottom {
            self.lock_to_bottom = true;
            self.vertical_scroll = count.saturating_sub(visible_lines);
        }
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(count);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);

        let p = Paragraph::new(
            raw_nmea
                .iter()
                .skip(self.vertical_scroll)
                .take(self.vertical_scroll + visible_lines)
                .map(|f| Line::from(f.2.as_str()))
                .collect::<Vec<Line>>(),
        );

        f.render_widget(p, area);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut self.vertical_scroll_state,
        );
    }

    fn hint(&mut self) -> &'static [&'static str] { &["`b` Lock to Bottom", "`↑↓` Scroll"] }
}
