use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(2), //Bottom
        ])
        .split(f.area());

    //header
    let header = ["Info", "NMEA", "Settings"]
        .iter()
        .cloned()
        .map(Span::from)
        .collect::<Vec<_>>();
    let tabs = Tabs::new(header)
        .select(app.tab.index())
        .block(Block::default().title("Term-NMEA").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    //content
    f.render_widget(tabs, chunks[0]);
    app.draw(f, chunks[1]);

    //footer
    let footer = Paragraph::new(app.hint())
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}
