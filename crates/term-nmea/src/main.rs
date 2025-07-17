use std::io::stdout;
use std::time::Duration;

use clap::Parser;
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
mod cli;
mod serial;
mod settings;
mod tab;
mod ui;
/// Entry point of the async TUI application
#[tokio::main]
async fn main() -> miette::Result<()> {
    // Parse CLI arguments
    let cli = cli::CliArgs::parse();

    // Load settings from TOML, overridden by CLI arguments
    Settings::init(&cli)?;

    // Enable raw mode and enter alternate screen for TUI
    enable_raw_mode().into_diagnostic()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).into_diagnostic()?;

    // Set up terminal with Crossterm backend
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).into_diagnostic()?;

    // Create async channel for receiving serial data
    let (tx, mut rx) = mpsc::channel(100);

    // Initialize the application state
    let mut app = app::App::new()?;

    // Spawn async task to read from serial port
    tokio::spawn(serial::start_serial_reader(
        Settings::port(),
        Settings::baud_rate(),
        tx,
    ));

    // Main TUI loop
    loop {
        // Redraw the UI
        terminal.draw(|f| ui::draw(f, &mut app)).into_diagnostic()?;

        // Handle input and serial updates concurrently
        tokio::select! {
            // Poll for keyboard/mouse events
            maybe_evt = poll_event(Duration::from_millis(10)) => {
                if let Ok(Some(evt)) = maybe_evt {
                    match evt {
                        Event::Key(key) => {
                            // Handle keyboard input; break loop on exit signal
                            if app.handle_key(key) {
                                break;
                            }
                        }
                        Event::Mouse(mouse_evt) => {
                            // Handle mouse input
                            app.handle_mouse(mouse_evt);
                        }
                        _ => {}
                    }
                }
            }

            // Handle incoming serial data
            Some((talker, identifier, sentence)) = rx.recv() => {
                app.update(talker, identifier, sentence);
            }
        }
    }

    // Restore terminal state before exiting
    disable_raw_mode().into_diagnostic()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).into_diagnostic()?;
    terminal.show_cursor().into_diagnostic()?;

    // Save settings
    Settings::save();
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
