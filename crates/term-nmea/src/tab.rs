use std::collections::VecDeque;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::Frame;

pub use crate::tab::info::TabInfo;
pub use crate::tab::nmea::TabNmea;
pub use crate::tab::settings::TabSettings;

mod info;
mod nmea;
mod settings;

pub trait ITab: Default {
    fn handle_key(&mut self, key: KeyEvent);
    fn handle_mouse(&mut self, mouse: MouseEvent);
    fn draw(&mut self, f: &mut Frame, area: ratatui::layout::Rect, raw_nmea: &VecDeque<String>);
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
