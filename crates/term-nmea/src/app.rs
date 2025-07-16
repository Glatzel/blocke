use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent};

use crate::settings::Settings;
use crate::tab::{ITab, NULL_CTX, Tab, TabInfo, TabNmea, TabSettings};

pub struct App {
    pub raw_nmea: VecDeque<String>,
    pub settings: Settings,
    pub tab: Tab,
    pub tab_info: TabInfo,
    pub tab_nmea: TabNmea,
    pub tab_settings: TabSettings,
}

impl App {
    pub fn new() -> miette::Result<Self> {
        let settings = Settings::init()?;
        Ok(Self {
            raw_nmea: VecDeque::with_capacity(settings.capacity),
            settings: settings,
            tab: Tab::Info,
            tab_info: TabInfo::default(),
            tab_nmea: TabNmea::default(),
            tab_settings: TabSettings::default(),
        })
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
    pub fn draw(&mut self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        match self.tab {
            Tab::Info => self.tab_info.draw(f, area, &NULL_CTX),
            Tab::Nmea => self.tab_nmea.draw(f, area, &self.raw_nmea),
            Tab::Settings => self.tab_settings.draw(f, area, &NULL_CTX),
        }
    }
    pub fn push(&mut self, sentence: String) {
        if self.raw_nmea.len() > self.settings.capacity {
            self.raw_nmea.pop_front();
        }
        self.raw_nmea.push_back(sentence);
    }
}
