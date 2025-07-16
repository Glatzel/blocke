use crossterm::event::{KeyCode, KeyEvent};

pub enum Tab {
    Info,
    Nmea,
    Settings,
}

pub struct App {
    pub tab: Tab,
}

impl App {
    pub async fn new() -> Self { Self { tab: Tab::Info } }

    pub async fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => return true,
            KeyCode::Right => self.next_tab().await,
            KeyCode::Left => self.prev_tab().await,
            _ => {}
        }
        false
    }

    async fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Info => Tab::Nmea,
            Tab::Nmea => Tab::Settings,
            Tab::Settings => Tab::Info,
        }
    }

    async fn prev_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Info => Tab::Settings,
            Tab::Nmea => Tab::Info,
            Tab::Settings => Tab::Nmea,
        }
    }
}
