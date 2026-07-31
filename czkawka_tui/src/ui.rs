use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};

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

                    ListItem::new(Line::from(vec![
                        Span::styled(prefix, style),
                        Span::raw(p.file_name().unwrap_or(p.as_os_str()).to_string_lossy().to_string()),
                    ]))
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
            let mut list_items = Vec::new();

            let filtered = tool.filtered_items();
            let mut current_group = None;

            for (actual_idx, item) in &filtered {
                let (g_idx, _) = *actual_idx;
                if current_group != Some(g_idx) {
                    current_group = Some(g_idx);
                    list_items.push(ListItem::new(Line::from(vec![Span::styled(
                        format!("--- Group {} ({} items) ---", g_idx + 1, tool.groups[g_idx].items.len()),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )])));
                }

                let prefix = if tool.selected_items.contains(actual_idx) { "[X] " } else { "[ ] " };
                let size_str = format!("{:>10} ", item.size); // Basic formatting for size
                list_items.push(ListItem::new(Line::from(vec![
                    Span::styled(
                        prefix,
                        Style::default().fg(if tool.selected_items.contains(actual_idx) { Color::Green } else { Color::DarkGray }),
                    ),
                    Span::styled(size_str, Style::default().fg(Color::Cyan)),
                    Span::raw(item.path.clone()),
                ])));
            }

            let items_list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title(format!("Results ({})", app.scan_dir)))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(items_list, chunks[2], &mut app.tools[tool_idx].list_state);
        }
    }

    // 4. Search Bar
    let input_title = match app.input_mode {
        InputMode::Normal => "Press '/' to filter, Enter to Scan, Esc to Cancel, 's' Select, 'd' Dir Picker",
        InputMode::Search => "Filter (Press Enter/Esc to stop)",
        InputMode::Select => "Select mode (a: All, n: None, i: Invert, b: All Exc Biggest, s: Exc Smallest, w: Exc Newest, o: Exc Oldest, Esc to cancel)",
        InputMode::DirPicker => "Dir Picker: Space -> Include, 'r' -> Reference, Enter -> Open, Esc -> Done",
        InputMode::ConfirmAction => "Confirm Action (y/n)",
    };

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Search => Style::default().fg(Color::Yellow),
        InputMode::Select => Style::default().fg(Color::Green),
        InputMode::DirPicker => Style::default().fg(Color::Cyan),
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
    let shortcut_text = "Enter Scan | Space Toggle | / Filter | s Select | F8 Delete | L Symlink | H Hardlink | e Export | F10/q Quit | j/k Nav | Tab Tool";
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
