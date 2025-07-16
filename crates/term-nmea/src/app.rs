use crossterm::event::{KeyCode, KeyEvent};

use crate::tab::{ITab, Tab, TabInfo, TabNmea, TabSettings};

pub struct App {
    pub tab: Tab,
    pub tab_info: TabInfo,
    pub tab_nmea: TabNmea,
    pub tab_settings: TabSettings,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tab::Info,
            tab_info: TabInfo::default(),
            tab_nmea: TabNmea::default(),
            tab_settings: TabSettings::default(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match (self.tab, key.code) {
            //global key
            (_, KeyCode::Right) => self.next_tab(),
            (_, KeyCode::Left) => self.prev_tab(),

            (_, KeyCode::Esc) => return true,

            //tab key
            (Tab::Info, k) => self.tab_info.handle_key(k),
            (Tab::Nmea, k) => self.tab_nmea.handle_key(k),
            (Tab::Settings, k) => self.tab_settings.handle_key(k),
        }
        false
    }
    fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Info => Tab::Nmea,
            Tab::Nmea => Tab::Settings,
            Tab::Settings => Tab::Info,
        }
    }

    fn prev_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Info => Tab::Settings,
            Tab::Nmea => Tab::Info,
            Tab::Settings => Tab::Nmea,
        }
    }
}
