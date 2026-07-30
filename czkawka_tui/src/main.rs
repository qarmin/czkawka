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
        &args[1]
    } else {
        "."
    };
    let scanner = Scanner::new(scan_dir);

    // Initial loading message
    app.all_items = vec![format!("Scanning {} for duplicates...", scan_dir)];

    let res = run_app(&mut terminal, &mut app, &scanner);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, scanner: &Scanner) -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut scanned = false;

    loop {
        terminal.draw(|f| {
            ui(f, app);
        })?;

        // Check if scan is complete
        if !scanned {
            if let Ok(results) = scanner.receiver.try_recv() {
                app.all_items = results;
                if app.all_items.is_empty() {
                    app.all_items.push("No duplicates found.".to_string());
                }
                scanned = true;
                app.list_state.select(Some(0)); // Initialize selection
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
                        KeyCode::F(8) => {
                            app.delete_selected();
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.next();
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.previous();
                        }
                        KeyCode::Char('/') | KeyCode::F(5) => {
                            app.input_mode = InputMode::Search;
                        }
                        _ => {}
                    },
                    InputMode::Search => match key.code {
                        KeyCode::Enter | KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.filter.push(c);
                            app.on_filter_change();
                        }
                        KeyCode::Backspace => {
                            app.filter.pop();
                            app.on_filter_change();
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
