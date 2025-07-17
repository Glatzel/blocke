use std::collections::VecDeque;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent};
use rax_nmea::data::{Identifier, Talker};

use crate::settings::Settings;
use crate::tab::{ITab, Tab, TabInfo, TabNmea, TabSettings};

pub struct App {
    pub raw_nmea: VecDeque<(Talker, Identifier, String)>,

    pub tab: Tab,
    pub tab_info: TabInfo,
    pub tab_nmea: TabNmea,
    pub tab_settings: TabSettings,
}

impl App {
    pub fn new() -> miette::Result<Self> {
        Ok(Self {
            raw_nmea: VecDeque::with_capacity(Settings::capacity()),
            tab: Tab::Info,
            tab_info: TabInfo::default(),
            tab_nmea: TabNmea::default(),
            tab_settings: TabSettings::default(),
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match (self.tab, key) {
            //global key
            (
                _,
                KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                },
            ) => self.next_tab(),
            (
                _,
                KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                },
            ) => self.prev_tab(),

            (
                _,
                KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                },
            ) => return true,

            //tab key
            (Tab::Info, k) => self.tab_info.handle_key(k),
            (Tab::Nmea, k) => self.tab_nmea.handle_key(k),
            (Tab::Settings, k) => self.tab_settings.handle_key(k),
        }
        false
    }
    pub fn handle_mouse(&mut self, mouse: MouseEvent) -> bool {
        match (self.tab, mouse) {
            (Tab::Info, mouse) => self.tab_info.handle_mouse(mouse),
            (Tab::Nmea, mouse) => self.tab_nmea.handle_mouse(mouse),
            (Tab::Settings, mouse) => self.tab_settings.handle_mouse(mouse),
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
            Tab::Info => self.tab_info.draw(f, area, &self.raw_nmea),
            Tab::Nmea => self.tab_nmea.draw(f, area, &self.raw_nmea),
            Tab::Settings => self.tab_settings.draw(f, area, &self.raw_nmea),
        }
    }
    pub fn update(&mut self, talker: Talker, identifier: Identifier, sentence: String) {
        if self.raw_nmea.len() > Settings::capacity() {
            self.raw_nmea.pop_front();
        }
        self.raw_nmea.push_back((talker, identifier, sentence));
    }
    pub fn hint(&mut self) -> String {
        const GLOBAL_HINT: [&str; 2] = ["`←/→` Tab", "`esc` Quit"];
        let tab_hint = match self.tab {
            Tab::Info => self.tab_info.hint(),
            Tab::Nmea => self.tab_nmea.hint(),
            Tab::Settings => self.tab_settings.hint(),
        };
        GLOBAL_HINT
            .iter()
            .chain(tab_hint.iter())
            .copied()
            .collect::<Vec<&str>>()
            .join(" | ")
    }
}
