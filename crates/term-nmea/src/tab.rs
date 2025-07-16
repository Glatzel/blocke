use crossterm::event::KeyCode;
use ratatui::widgets::Widget;

pub use crate::tab::info::TabInfo;
pub use crate::tab::nmea::TabNmea;
pub use crate::tab::settings::TabSettings;

mod info;
mod nmea;
mod settings;

pub trait ITab: Widget {
    fn handle_key(&mut self, key: KeyCode);
}
#[derive(Clone, Debug, Copy)]
pub enum Tab {
    Info,
    Nmea,
    Settings,
}
