use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::app::{App, Tab};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(2), //Bottom
        ])
        .split(f.area());

    let titles = ["Info", "NMEA", "Settings"]
        .iter()
        .cloned()
        .map(Span::from)
        .collect::<Vec<_>>();

    let tab_index = match app.tab {
        Tab::Info => 0,
        Tab::Nmea => 1,
        Tab::Settings => 2,
    };

    let tabs = Tabs::new(titles)
        .select(tab_index)
        .block(Block::default().title("Term-NMEA").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(tabs, chunks[0]);

    match app.tab {
        Tab::Info => draw_info(f, app, chunks[1]),
        Tab::Nmea => draw_nmea(f, app, chunks[1]),
        Tab::Settings => draw_settings(f, chunks[1]),
    }
    let footer = Paragraph::new("`←/→` Tab | `esc` Quit")
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}
fn draw_info(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let p = Paragraph::new("Info go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
        .block(Block::default().title("Info"));

    f.render_widget(p, area);
}
fn draw_nmea(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let p = Paragraph::new("NMEA go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
        .block(Block::default().title("NMEA"));
    f.render_widget(p, area);
}

fn draw_settings(f: &mut Frame, area: ratatui::layout::Rect) {
    let p = Paragraph::new("Settings go here.\nPress ← → to switch tabs.\nPress 'q' to quit.")
        .block(Block::default().title("Settings"));
    f.render_widget(p, area);
}
