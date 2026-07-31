mod app;
mod ui;
mod scanner;

use std::io;
use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use crate::app::{App, InputMode};
use crate::ui::ui;
use crate::scanner::Scanner;
use std::env;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    // Read dir from args or default to current directory
    let args: Vec<String> = env::args().collect();
    let scan_dir = if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string()
    };
    app.scan_dir = scan_dir.clone();

    let mut scanner = Scanner::new();

    // Start initial scan for the first tool
    {
        let tool = app.active_tool_mut();
        tool.all_items = vec![format!("Press Enter to scan {}...", scan_dir)];
    }

    let res = run_app(&mut terminal, &mut app, &mut scanner);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
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
        if let Ok((tool_type, results)) = scanner.receiver.try_recv() {
            if let Some(tool) = app.tools.iter_mut().find(|t| t.tool_type == tool_type) {
                tool.all_items = results;
                if tool.all_items.is_empty() {
                    tool.all_items.push("No items found.".to_string());
                }
                tool.list_state.select(Some(0)); // Initialize selection
                tool.selected_items.clear();
            }
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::F(10) => return Ok(()),
                        KeyCode::F(8) | KeyCode::Delete => {
                            app.delete_selected();
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
                        KeyCode::Char(' ') => {
                            app.toggle_selection();
                            app.next();
                        }
                        KeyCode::Char('/') => {
                            app.input_mode = InputMode::Search;
                        }
                        KeyCode::Char('s') => {
                            app.input_mode = InputMode::Select;
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
                            app.active_tool_mut().all_items = vec!["Scanning...".to_string()];
                            app.active_tool_mut().selected_items.clear();
                            scanner.start_scan(tt, &app.scan_dir);
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
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
