use czkawka_core::common::config_cache_path::set_config_cache_path;
mod app;
mod scanner;
mod ui;

use std::time::{Duration, Instant};
use std::{env, io};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

use crate::app::{App, InputMode};
use crate::scanner::Scanner;
use crate::ui::ui;

fn main() -> Result<()> {
    set_config_cache_path("czkawka_tui", "czkawka_tui");
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    // Read dir from args or default to current directory
    let args: Vec<String> = env::args().collect();
    let scan_dir = if args.len() > 1 { args[1].clone() } else { ".".to_string() };
    app.scan_dir = scan_dir.clone();

    let mut scanner = Scanner::new();

    // Start initial scan for the first tool
    {
        let tool = app.active_tool_mut();
        tool.groups = vec![crate::app::TuiGroup {
            items: vec![crate::app::TuiItem {
                path: format!("Press Enter to scan {}...", scan_dir),
                size: 0,
                modified_date: 0,
            }],
        }];
    }

    let res = run_app(&mut terminal, &mut app, &mut scanner);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, scanner: &mut Scanner) -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            ui(f, app);
        })?;

        // Check if scan is complete
        if let Ok((tool_type, results)) = scanner.receiver.try_recv()
            && let Some(tool) = app.tools.iter_mut().find(|t| t.tool_type == tool_type)
        {
            tool.groups = results;
            if tool.groups.is_empty() {
                tool.groups.push(crate::app::TuiGroup {
                    items: vec![crate::app::TuiItem {
                        path: "No items found.".to_string(),
                        size: 0,
                        modified_date: 0,
                    }],
                });
                app.status_message = "Scan completed. No results.".to_string();
            } else {
                app.status_message = "Scan completed.".to_string();
            }
            tool.table_state.select(Some(0)); // Initialize selection
            tool.selected_items.clear();
        }

        // Check progress
        while let Ok(progress) = scanner.progress_receiver.try_recv() {
            let display = progress.to_display();
            if display.all_progress >= 0 {
                app.status_message = format!("{} [{}%]", display.label, display.all_progress);
            } else {
                app.status_message = display.label.clone();
            }
        }

        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::F(10) => return Ok(()),
                    KeyCode::Char('a') => {
                        app.input_mode = InputMode::ActionMenu;
                    }
                    KeyCode::Char('e') => {
                        app.export_selected();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.previous();
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        app.previous_tool();
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        app.next_tool();
                    }
                    KeyCode::PageDown => {
                        app.page_down();
                    }
                    KeyCode::PageUp => {
                        app.page_up();
                    }
                    KeyCode::Home => {
                        app.home();
                    }
                    KeyCode::End => {
                        app.end();
                    }
                    KeyCode::Char(' ') => {
                        app.toggle_selection();
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Search;
                    }
                    KeyCode::Char('s') => {
                        app.input_mode = InputMode::Select;
                    }
                    KeyCode::Char('o') => {
                        app.open_dir_picker();
                    }
                    KeyCode::Char('O') => {
                        let tool = app.active_tool_mut();
                        tool.cycle_sort_order();
                        app.status_message = format!("Sort order: {:?}", tool.sort_order);
                    }
                    KeyCode::Tab => {
                        app.next_tool();
                    }
                    KeyCode::BackTab => {
                        app.previous_tool();
                    }
                    KeyCode::Enter => {
                        // Start scan for active tool
                        let tt = app.active_tool().tool_type;
                        app.active_tool_mut().groups = vec![crate::app::TuiGroup {
                            items: vec![crate::app::TuiItem {
                                path: "Scanning...".to_string(),
                                size: 0,
                                modified_date: 0,
                            }],
                        }];
                        app.active_tool_mut().selected_items.clear();
                        let mut inc = app.selected_directories.clone();
                        if inc.is_empty() {
                            inc.push(app.scan_dir.clone());
                        }
                        scanner.start_scan(tt, inc, app.reference_directories.clone());
                    }
                    KeyCode::Esc => {
                        scanner.cancel();
                        app.status_message = "Scan cancelled".to_string();
                    }
                    _ => {}
                },
                InputMode::Search => match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        let tool = app.active_tool_mut();
                        tool.filter.push(c);
                        app.on_filter_change();
                    }
                    KeyCode::Backspace => {
                        let tool = app.active_tool_mut();
                        tool.filter.pop();
                        app.on_filter_change();
                    }
                    _ => {}
                },
                InputMode::Select => match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('a') => {
                        app.select_all();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('n') => {
                        app.deselect_all();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('i') => {
                        app.invert_selection();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('b') => {
                        app.select_all_except_biggest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('s') => {
                        app.select_all_except_smallest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('w') => {
                        app.select_all_except_newest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('o') => {
                        app.select_all_except_oldest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('B') => {
                        app.select_one_biggest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('S') => {
                        app.select_one_smallest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('W') => {
                        app.select_one_newest();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('O') => {
                        app.select_one_oldest();
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::DirPicker => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.dir_picker_next();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.dir_picker_previous();
                    }
                    KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
                        app.dir_picker_enter();
                    }
                    KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => {
                        if let Some(parent) = app.dir_picker_current_path.parent() {
                            app.dir_picker_current_path = parent.to_path_buf();
                            app.refresh_dir_picker();
                        }
                    }
                    KeyCode::Char(' ') => {
                        app.dir_picker_add_included();
                    }
                    KeyCode::Char('r') => {
                        app.dir_picker_add_reference();
                    }
                    _ => {}
                },
                InputMode::ActionMenu => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('d') | KeyCode::Delete => {
                        app.pending_action = crate::app::PendingAction::Delete;
                        app.input_mode = InputMode::ConfirmAction;
                    }
                    KeyCode::Char('s') => {
                        app.pending_action = crate::app::PendingAction::Symlink;
                        app.input_mode = InputMode::ConfirmAction;
                    }
                    KeyCode::Char('h') => {
                        app.pending_action = crate::app::PendingAction::Hardlink;
                        app.input_mode = InputMode::ConfirmAction;
                    }
                    _ => {}
                },
                InputMode::ConfirmAction => match key.code {
                    KeyCode::Char('y') | KeyCode::Enter => {
                        app.execute_action();
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char('n') | KeyCode::Esc => {
                        app.status_message = "Action cancelled".to_string();
                        app.input_mode = InputMode::Normal;
                        app.pending_action = crate::app::PendingAction::None;
                    }
                    _ => {}
                },
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
