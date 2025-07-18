use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};
use rax_nmea::data::{Identifier, Talker};

use crate::settings::{Settings, SETTINGS};

#[derive(Default)]
pub struct TabSettings {}
impl super::ITab for TabSettings {
    fn handle_key(&mut self, _key: KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        _raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) {
        let settings = SETTINGS.get().unwrap();
        let lines = vec![
            Line::from(vec![Span::raw(format!("Port: {}", settings.port))]),
            Line::from(vec![Span::raw(format!("Baudrate: {}", settings.baud_rate))]),
            Line::from(vec![Span::raw(format!("Capacity: {}", settings.capacity))]),
            // Add more fields as needed
        ];

        let paragraph = Paragraph::new(lines).block(Block::default());

        f.render_widget(paragraph, area);
    }

    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
