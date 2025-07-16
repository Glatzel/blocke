use std::io::stdout;
use std::time::Duration;

use app::{App, Tab};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, *};
use miette::IntoDiagnostic;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use serial::start_serial_reader;
use tokio::sync::mpsc;
use tokio::task;

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
    let mut app = App::new().await;

    tokio::spawn(serial::start_serial_reader(
        config.port.clone(),
        config.baud_rate,
        tx,
    ));

    loop {
        terminal.draw(|f| ui::draw(f, &app)).into_diagnostic()?;

        tokio::select! {
            maybe_key = poll_key(Duration::from_millis(50)) => {
                if let Ok(Some(key)) = maybe_key {
                    if app.handle_key(key).await { break; }
                }
            }
            Some(data) = rx.recv() => { /* … */ }
        }
    }

    disable_raw_mode().into_diagnostic()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).into_diagnostic()?;
    terminal.show_cursor().into_diagnostic()?;
    Ok(())
}
async fn poll_key(timeout: Duration) -> std::io::Result<Option<crossterm::event::KeyEvent>> {
    task::spawn_blocking(move || {
        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(k) = crossterm::event::read()? {
                return Ok(Some(k));
            }
        }
        Ok(None)
    })
    .await
    .expect("join error")
}
