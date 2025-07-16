use std::io::stdout;
use std::time::Duration;

use app::App;
use crossterm::event::Event;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, *};
use miette::IntoDiagnostic;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc;
use tokio::task;

use crate::settings::Settings;
mod app;
mod serial;
mod settings;
mod tab;
mod ui;

#[tokio::main]
async fn main() -> miette::Result<()> {
    enable_raw_mode().into_diagnostic()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).into_diagnostic()?;

    let config = Settings::init()?;

    let (tx, mut rx) = mpsc::channel(100);
    let mut app = App::new()?;

    tokio::spawn(serial::start_serial_reader(
        config.port.clone(),
        config.baud_rate,
        tx,
    ));

    loop {
        terminal.draw(|f| ui::draw(f, &mut app)).into_diagnostic()?;

        tokio::select! {
           maybe_evt = poll_event(Duration::from_millis(10)) => {
                if let Ok(Some(evt)) = maybe_evt {
                    match evt {
                        crossterm::event::Event::Key(key) => {
                            if app.handle_key(key) {
                                break;
                            }
                        }
                        crossterm::event::Event::Mouse(mouse_evt) => {
                            app.handle_mouse(mouse_evt);
                        }
                        _ => {}
                    }
                }
            }
            Some((talker, identifier, sentence)) = rx.recv() => {
                app.update(talker, identifier, sentence)
            }
        }
    }

    disable_raw_mode().into_diagnostic()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).into_diagnostic()?;
    terminal.show_cursor().into_diagnostic()?;
    Ok(())
}
async fn poll_event(timeout: Duration) -> std::io::Result<Option<Event>> {
    task::spawn_blocking(move || {
        if crossterm::event::poll(timeout)? {
            return crossterm::event::read().map(Some);
        }
        Ok(None)
    })
    .await
    .expect("join error")
}
