use std::io::stdout;

use app::{App, Tab};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, *};
use miette::IntoDiagnostic;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use serial::start_serial_reader;
use tokio::sync::mpsc;

use crate::config::Config;
mod app;
mod config;
mod event;
mod serial;
mod ui;
#[tokio::main]
async fn main() -> miette::Result<()> {
    enable_raw_mode().into_diagnostic()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).into_diagnostic()?;

    let config = Config::init()?;

    let (tx, mut rx) = mpsc::channel(100);
    let mut app = App::new();

    tokio::spawn(serial::start_serial_reader(
        &config.port,
        config.baud_rate,
        tx,
    ));

    loop {
        terminal.draw(|f| ui::draw(f, &app)).into_diagnostic()?;

        tokio::select! {
            Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(50)) => {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if app.handle_key(key) {
                        break;
                    }
                }
            }
            Some(data) = rx.recv().await => {
               continue;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
