use std::collections::VecDeque;

use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Paragraph};
use rax_nmea::data::{Identifier, Talker};

use crate::settings::SETTINGS;

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
    ) -> miette::Result<()> {
        let toml_str =
            toml::to_string_pretty(SETTINGS.get().unwrap()).expect("TOML serialize error: {e}");
        let paragraph = Paragraph::new(toml_str)
            .block(Block::default())
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, area);
        Ok(())
    }
    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
