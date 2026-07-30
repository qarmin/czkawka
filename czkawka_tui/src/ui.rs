use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, InputMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top bar
            Constraint::Min(1),    // Main content
            Constraint::Length(3), // Input / Search
            Constraint::Length(1), // Shortcut bar
        ])
        .split(f.size());

    // 1. Top Bar
    let title_text = if app.status_message.is_empty() {
        "Czkawka TUI - Duplicate Finder".to_string()
    } else {
        format!("Czkawka TUI - {}", app.status_message)
    };
    let title = Paragraph::new(Line::from(vec![
        Span::styled(title_text, Style::default().add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // 2. Main Content
    let items: Vec<ListItem> = app
        .filtered_items()
        .iter()
        .map(|s| {
            ListItem::new(Line::from(Span::raw(s.to_string())))
        })
        .collect();

    let items_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(items_list, chunks[1], &mut app.list_state);

    // 3. Search Bar
    let input_title = match app.input_mode {
        InputMode::Normal => "Press '/' to search",
        InputMode::Search => "Search (Press Enter/Esc to stop)",
    };

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Search => Style::default().fg(Color::Yellow),
    };

    let input = Paragraph::new(app.filter.as_str())
        .style(input_style)
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[2]);

    if matches!(app.input_mode, InputMode::Search) {
        f.set_cursor(
            chunks[2].x + app.filter.len() as u16 + 1,
            chunks[2].y + 1,
        );
    }

    // 4. FAR-style bottom bar
    let shortcut_text = "F1 Help | F5 Search | F8 Delete | F10 Quit | j/k Navigate";
    let shortcuts = Paragraph::new(shortcut_text)
        .style(Style::default().bg(Color::Cyan).fg(Color::Black));
    f.render_widget(shortcuts, chunks[3]);
}
