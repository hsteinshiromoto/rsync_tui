mod app;
mod event;
mod path;
mod rsync;
mod ui;

use std::io;
use app::{App, Mode, Panel};
use crossterm::{
    event::{KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let mut app = App::new();
    let result = run(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| ui::layout::render(frame, app))?;

        if let Some(key) = event::poll_event(100)? {
            // Global commands (Ctrl+key, work in both modes)
            let handled = match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.should_quit = true;
                    true
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    run_rsync(app, false);
                    true
                }
                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    run_rsync(app, true);
                    true
                }
                _ => false,
            };

            // Mode-specific handling (if not handled globally)
            if !handled {
                match app.mode {
                    Mode::Normal => handle_normal_mode(app, &key),
                    Mode::Insert => handle_insert_mode(app, &key),
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_normal_mode(app: &mut App, key: &crossterm::event::KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') => app.should_quit = true,

        // Panel navigation with Tab/Shift+Tab
        KeyCode::Tab => app.next_panel(),
        KeyCode::BackTab => app.prev_panel(),

        // Panel navigation shortcuts (1-4)
        KeyCode::Char('1') => app.active_panel = Panel::Source,
        KeyCode::Char('2') => app.active_panel = Panel::Destination,
        KeyCode::Char('3') => app.active_panel = Panel::Options,
        KeyCode::Char('4') => app.active_panel = Panel::Logs,

        // Vim-style navigation (j/k)
        KeyCode::Char('j') => app.next_panel(), // Move down
        KeyCode::Char('k') => app.prev_panel(), // Move up

        // Enter insert mode (only in Source/Destination panels)
        KeyCode::Char('i')
            if matches!(app.active_panel, Panel::Source | Panel::Destination) =>
        {
            app.mode = Mode::Insert;
        }

        // Option toggles with letter keys
        KeyCode::Char('a') => app.options.toggle(0), // Archive
        KeyCode::Char('v') => app.options.toggle(1), // Verbose
        KeyCode::Char('z') => app.options.toggle(2), // Compress
        KeyCode::Char('n') => app.options.toggle(3), // Dry-run
        KeyCode::Char('p') => app.options.toggle(4), // Progress
        KeyCode::Char('d') => app.options.toggle(5), // Delete
        KeyCode::Char('h') => app.options.toggle(6), // Human-readable
        KeyCode::Char('e') => app.options.toggle(7), // SSH

        _ => {}
    }
}

fn handle_insert_mode(app: &mut App, key: &crossterm::event::KeyEvent) {
    match key.code {
        // Exit insert mode
        KeyCode::Esc => app.mode = Mode::Normal,

        // Tab - path autocomplete
        KeyCode::Tab => {
            let current_path = match app.active_panel {
                Panel::Source => app.source.clone(),
                Panel::Destination => app.destination.clone(),
                _ => return,
            };

            if let Some(completed) = path::complete_path(&current_path) {
                match app.active_panel {
                    Panel::Source => app.source = completed,
                    Panel::Destination => app.destination = completed,
                    _ => {}
                }
            }
        }

        // Text input (allow Shift for uppercase)
        KeyCode::Char(c)
            if !key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
        {
            match app.active_panel {
                Panel::Source => app.source.push(c),
                Panel::Destination => app.destination.push(c),
                _ => {}
            }
        }

        // Backspace
        KeyCode::Backspace => {
            match app.active_panel {
                Panel::Source => { app.source.pop(); }
                Panel::Destination => { app.destination.pop(); }
                _ => {}
            }
        }

        _ => {}
    }
}

fn run_rsync(app: &mut App, dry_run: bool) {
    use std::process::Command;
    use crate::rsync::command::build_command;

    let mut opts = app.options.clone();
    if dry_run {
        opts.dry_run = true;
    }

    let args = build_command(&app.source, &app.destination, &opts);
    app.log(format!("Running: {}", args.join(" ")));

    // Execute rsync (skip first element which is "rsync")
    let output = Command::new("rsync")
        .args(&args[1..])
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);

            for line in stdout.lines() {
                app.log(line.to_string());
            }
            for line in stderr.lines() {
                app.log(format!("[ERR] {}", line));
            }

            if out.status.success() {
                app.log("Sync completed successfully".to_string());
            } else {
                app.log(format!("Sync failed with exit code: {:?}", out.status.code()));
            }
        }
        Err(e) => {
            app.log(format!("Failed to execute rsync: {}", e));
        }
    }
}
