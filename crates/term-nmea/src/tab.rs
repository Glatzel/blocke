use std::collections::VecDeque;

pub use coord::TabCoord;
use crossterm::event::{KeyEvent, MouseEvent};
pub use info::TabInfo;
pub use nmea::TabNmea;
use ratatui::Frame;
use rax_nmea::data::{Identifier, Talker};
pub use settings::TabSettings;

mod coord;
mod info;
mod nmea;
mod settings;
pub trait ITab: Default {
    fn handle_key(&mut self, key: KeyEvent);
    fn handle_mouse(&mut self, mouse: MouseEvent);
    fn draw(
        &mut self,
        f: &mut Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    );
    fn hint(&mut self) -> &'static [&'static str];
}
#[derive(Clone, Debug, Copy)]
pub enum Tab {
    Info,
    Coord,
    Nmea,
    Settings,
}
impl Tab {
    pub fn index(&self) -> usize {
        match self {
            Tab::Info => 0,
            Tab::Coord => 1,
            Tab::Nmea => 2,
            Tab::Settings => 3,
        }
    }
}
