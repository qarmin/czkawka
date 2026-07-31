use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use crate::app::{App, InputMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top bar (Tabs)
            Constraint::Length(3), // Status Message / Title
            Constraint::Min(1),    // Main content
            Constraint::Length(3), // Input / Search
            Constraint::Length(1), // Shortcut bar
        ])
        .split(f.size());

    // 1. Top Bar (Tabs)
    let titles: Vec<Line> = app.tools.iter().map(|t| {
        Line::from(t.tool_type.name())
    }).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tools"))
        .select(app.active_tool_idx)
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[0]);

    // 2. Status Message / Title
    let title_text = if app.status_message.is_empty() {
        format!("Czkawka TUI - {}", app.active_tool().tool_type.name())
    } else {
        format!("Czkawka TUI - {}", app.status_message)
    };
    let title = Paragraph::new(Line::from(vec![
        Span::styled(title_text, Style::default().add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[1]);

    // 3. Main Content
    let tool_idx = app.active_tool_idx;
    let tool = &app.tools[tool_idx];
    let items: Vec<ListItem> = tool
        .filtered_items()
        .iter()
        .map(|(actual_idx, s)| {
            let prefix = if tool.selected_items.contains(actual_idx) { "[X] " } else { "[ ] " };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, Style::default().fg(if tool.selected_items.contains(actual_idx) { Color::Green } else { Color::DarkGray })),
                Span::raw(s.to_string()),
            ]))
        })
        .collect();

    let items_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!("Results ({})", app.scan_dir)))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(items_list, chunks[2], &mut app.tools[tool_idx].list_state);

    // 4. Search Bar
    let input_title = match app.input_mode {
        InputMode::Normal => "Press '/' to filter, Enter to Scan, Esc to Cancel, 's' for Select Mode",
        InputMode::Search => "Filter (Press Enter/Esc to stop)",
        InputMode::Select => "Select mode (a: All, n: None, i: Invert, Esc to cancel)",
    };

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Search => Style::default().fg(Color::Yellow),
        InputMode::Select => Style::default().fg(Color::Green),
    };

    let filter_text = app.active_tool().filter.as_str();
    let input = Paragraph::new(filter_text)
        .style(input_style)
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[3]);

    if matches!(app.input_mode, InputMode::Search) {
        f.set_cursor(
            chunks[3].x + filter_text.len() as u16 + 1,
            chunks[3].y + 1,
        );
    }

    // 5. FAR-style bottom bar
    let shortcut_text = "Enter Scan | Space Toggle | / Filter | s Select | F8/Del Delete | e Export | F10/q Quit | j/k Nav | Tab/S-Tab Tool";
    let shortcuts = Paragraph::new(shortcut_text)
        .style(Style::default().bg(Color::Cyan).fg(Color::Black));
    f.render_widget(shortcuts, chunks[4]);
}
