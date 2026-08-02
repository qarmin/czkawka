use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs};

use crate::app::{App, InputMode, ViewItem};

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
    let titles: Vec<Line> = app.tools.iter().map(|t| Line::from(t.tool_type.name())).collect();
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
    let title = Paragraph::new(Line::from(vec![Span::styled(title_text, Style::default().add_modifier(Modifier::BOLD))])).block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[1]);

    // 3. Main Content
    match app.input_mode {
        InputMode::DirPicker => {
            let title = format!("Directory Picker - {}", app.dir_picker_current_path.to_string_lossy());
            let items: Vec<ListItem> = app
                .dir_picker_items
                .iter()
                .map(|p| {
                    let p_str = p.to_string_lossy().to_string();
                    let mut prefix = "[ ] ";
                    let mut style = Style::default();
                    if app.selected_directories.contains(&p_str) {
                        prefix = "[I] ";
                        style = style.fg(Color::Green);
                    } else if app.reference_directories.contains(&p_str) {
                        prefix = "[R] ";
                        style = style.fg(Color::Yellow);
                    }

                    let display_name = if Some(p.as_path()) == app.dir_picker_current_path.parent() {
                        "..".to_string()
                    } else {
                        p.file_name().unwrap_or(p.as_os_str()).to_string_lossy().to_string()
                    };

                    ListItem::new(Line::from(vec![Span::styled(prefix, style), Span::raw(display_name)]))
                })
                .collect();

            let items_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(title))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            f.render_stateful_widget(items_list, chunks[2], &mut app.dir_picker_list_state);
        }
        _ => {
            let tool_idx = app.active_tool_idx;
            let tool = &app.tools[tool_idx];

            let filtered = tool.filtered_items();
            let mut rows = Vec::new();

            for view_item in &filtered {
                match view_item {
                    ViewItem::Header(g_idx, group) => {
                        rows.push(
                            Row::new(vec![
                                Cell::from(format!("--- Group {} ({} items) ---", g_idx + 1, group.items.len()))
                                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                                Cell::from(""),
                                Cell::from(""),
                                Cell::from(""),
                            ])
                            .bottom_margin(0),
                        );
                    }
                    ViewItem::Item(g_idx, i_idx, item) => {
                        let is_selected = tool.selected_items.contains(&(*g_idx, *i_idx));
                        let prefix = if is_selected { "[X]" } else { "[ ]" };

                        let size = item.size;
                        let size_str = humansize::format_size(size, humansize::BINARY);

                        let p = std::path::Path::new(&item.path);
                        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                        let path = p.parent().unwrap_or(std::path::Path::new("")).to_string_lossy().to_string();

                        let date = if item.modified_date > 0 {
                            // Format the system time. item.modified_date is a unix timestamp in seconds.
                            // However humansize is not sufficient here. We can use chrono, but for now let's just show it.
                            if let Some(dt) = chrono::DateTime::from_timestamp(item.modified_date as i64, 0) {
                                dt.format("%Y-%m-%d %H:%M:%S").to_string()
                            } else {
                                format!("{}", item.modified_date)
                            }
                        } else {
                            "".to_string()
                        };

                        rows.push(Row::new(vec![
                            Cell::from(format!("{} {}", prefix, name)).style(Style::default().fg(if is_selected { Color::Green } else { Color::White })),
                            Cell::from(path),
                            Cell::from(size_str).style(Style::default().fg(Color::Cyan)),
                            Cell::from(date),
                        ]));
                    }
                }
            }

            let header = Row::new(vec!["Name", "Path", "Size", "Modified"])
                .style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Magenta))
                .bottom_margin(1);

            let table = Table::new(
                rows,
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                ],
            )
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(format!("Results ({})", app.scan_dir)))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

            f.render_stateful_widget(table, chunks[2], &mut app.tools[tool_idx].table_state);
        }
    }

    // 4. Search Bar
    let input_title = match app.input_mode {
        InputMode::Normal => "Press '/' to filter, Enter to Scan, Esc to Cancel, 's' Select, 'o' Dir Picker",
        InputMode::Search => "Filter (Press Enter/Esc to stop)",
        InputMode::Select => "Select mode (a: All, n: None, i: Invert, b: All Exc Biggest, s: Exc Smallest, w: Exc Newest, o: Exc Oldest, Esc to cancel)",
        InputMode::ActionMenu => "Action menu (d: Delete, s: Symlink, h: Hardlink, Esc to cancel)",
        InputMode::DirPicker => "Dir Picker: Space -> Include, 'r' -> Reference, Enter -> Open, Esc -> Done",
        InputMode::ConfirmAction => "Confirm Action (y/n)",
    };

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Search => Style::default().fg(Color::Yellow),
        InputMode::Select => Style::default().fg(Color::Green),
        InputMode::DirPicker => Style::default().fg(Color::Cyan),
        InputMode::ActionMenu => Style::default().fg(Color::Magenta),
        InputMode::ConfirmAction => Style::default().fg(Color::Red),
    };

    let filter_text = app.active_tool().filter.as_str();
    let input = Paragraph::new(filter_text)
        .style(input_style)
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[3]);

    if matches!(app.input_mode, InputMode::Search) {
        f.set_cursor(chunks[3].x + filter_text.len() as u16 + 1, chunks[3].y + 1);
    }

    // 5. FAR-style bottom bar
    let shortcut_text = "Enter Scan | Space Toggle | / Filter | s Select | a Action | e Export | O Sort | o Dir | q Quit | j/k/h/l Nav";
    let shortcuts = Paragraph::new(shortcut_text).style(Style::default().bg(Color::Cyan).fg(Color::Black));
    f.render_widget(shortcuts, chunks[4]);

    // 6. Action Confirmation Modal
    if app.input_mode == InputMode::ConfirmAction {
        let tool = app.active_tool();
        let selected_count = tool.selected_items.len();

        let text = format!("Are you sure you want to {:?} {} items? (y/n)", app.pending_action, selected_count);

        let block = Block::default()
            .title("Confirm")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Red).fg(Color::White));
        let paragraph = Paragraph::new(text).block(block);

        // Center the popup
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Length(3), Constraint::Percentage(40)])
            .split(f.size());

        let popup_layout_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)])
            .split(popup_layout[1]);

        f.render_widget(ratatui::widgets::Clear, popup_layout_h[1]); // clear background
        f.render_widget(paragraph, popup_layout_h[1]);
    }
}
