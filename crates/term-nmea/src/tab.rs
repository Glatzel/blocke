use crossterm::event::KeyCode;
use ratatui::Frame;

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
impl Tab {
    pub fn index(&self) -> usize {
        match self {
            Tab::Info => 0,
            Tab::Nmea => 1,
            Tab::Settings => 2,
        }
    }
}
