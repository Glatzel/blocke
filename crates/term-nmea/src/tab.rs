use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::widgets::Widget;

pub use crate::tab::info::TabInfo;
pub use crate::tab::nmea::TabNmea;
pub use crate::tab::settings::TabSettings;

mod info;
mod nmea;
mod settings;

pub trait ITab {
    fn handle_key(&mut self, key: KeyCode);
    fn draw(&mut self, f: &mut Frame, area: ratatui::layout::Rect);
}
#[derive(Clone, Debug, Copy)]
pub enum Tab {
    Info,
    Nmea,
    Settings,
}
