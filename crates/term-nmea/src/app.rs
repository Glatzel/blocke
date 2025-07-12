use crossterm::event::{KeyCode, KeyEvent};

pub enum Tab {
    Nmea,
    Ubx,
    Settings,
}

pub struct App {
    pub tab: Tab,
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab: Tab::Nmea,
            logs: vec![],
        }
    }

    pub fn push_log(&mut self, line: String) {
        self.logs.push(line);
        if self.logs.len() > 200 {
            self.logs.remove(0);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return true,
            KeyCode::Right => self.next_tab(),
            KeyCode::Left => self.prev_tab(),
            _ => {}
        }
        false
    }

    fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Nmea => Tab::Ubx,
            Tab::Ubx => Tab::Settings,
            Tab::Settings => Tab::Nmea,
        }
    }

    fn prev_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Nmea => Tab::Settings,
            Tab::Ubx => Tab::Nmea,
            Tab::Settings => Tab::Ubx,
        }
    }
}