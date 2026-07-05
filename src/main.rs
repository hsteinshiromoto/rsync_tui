mod app;
mod config;
mod event;
mod path;
mod rsync;
mod ui;

use std::io;
use std::sync::mpsc::TryRecvError;

use app::{App, Confirm, Mode, Panel};
use config::Config;
use ratatui::crossterm::{
    event::{KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use rsync::command::{build_command, describe_exit_code};
use rsync::runner::{self, RsyncEvent};

fn main() -> anyhow::Result<()> {
    install_panic_hook();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app (load config if available)
    let mut app = App::new();
    if let Some(cfg) = Config::load() {
        app.source = cfg.source;
        app.source_cursor = app.source.len();
        app.destination = cfg.destination;
        app.dest_cursor = app.destination.len();
        app.options = cfg.options;
    }
    let result = run(&mut terminal, &mut app);

    // Save config on exit
    let _ = Config {
        source: app.source.clone(),
        destination: app.destination.clone(),
        options: app.options.clone(),
    }
    .save();

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

/// Restore the terminal before printing a panic so the shell stays usable
fn install_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, ratatui::crossterm::cursor::Show);
        original(info);
    }));
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    loop {
        drain_rsync_events(app);
        terminal.draw(|frame| ui::layout::render(frame, app))?;

        if let Some(key) = event::poll_event(100)? {
            // A confirmation modal captures all input while open
            if app.confirm.is_some() {
                handle_confirm_key(app, &key);
                continue;
            }

            // Global commands (Ctrl+key, work in both modes)
            let handled = match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if app.running {
                        app.confirm = Some(Confirm::Cancel);
                    } else {
                        app.should_quit = true;
                    }
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
            cancel_rsync(app);
            break;
        }
    }

    Ok(())
}

fn handle_normal_mode(app: &mut App, key: &ratatui::crossterm::event::KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') => app.should_quit = true,

        // Ask to cancel a running transfer
        KeyCode::Esc if app.running => app.confirm = Some(Confirm::Cancel),

        // Panel navigation with Tab/Shift+Tab
        KeyCode::Tab => app.next_panel(),
        KeyCode::BackTab => app.prev_panel(),

        // Panel navigation shortcuts (1-5)
        KeyCode::Char('1') => app.active_panel = Panel::Source,
        KeyCode::Char('2') => app.active_panel = Panel::Destination,
        KeyCode::Char('3') => app.active_panel = Panel::Options,
        KeyCode::Char('4') => app.active_panel = Panel::Logs,
        KeyCode::Char('5') => app.active_panel = Panel::Progress,

        // Vim-style navigation (j/k)
        KeyCode::Char('j') => app.next_panel(), // Move down
        KeyCode::Char('k') => app.prev_panel(), // Move up

        // Scroll logs (Page Up/Down in Normal mode)
        KeyCode::PageUp => app.scroll_logs_up(),
        KeyCode::PageDown => app.scroll_logs_down(),

        // Enter insert mode (only in Source/Destination panels)
        KeyCode::Char('i')
            if matches!(app.active_panel, Panel::Source | Panel::Destination) =>
        {
            app.mode = Mode::Insert;
        }

        // Execute rsync when Enter is pressed in Logs panel
        KeyCode::Enter if app.active_panel == Panel::Logs => {
            run_rsync(app, false);
        }

        // Option toggles via the shared OPTIONS table
        KeyCode::Char(c) => {
            app.options.toggle_key(c);
        }
        _ => {}
    }
}

fn handle_insert_mode(app: &mut App, key: &ratatui::crossterm::event::KeyEvent) {
    match key.code {
        // Exit insert mode
        KeyCode::Esc => app.mode = Mode::Normal,

        // Enter - move to next panel, stay in Insert if possible
        KeyCode::Enter => {
            app.next_panel();
            if !matches!(app.active_panel, Panel::Source | Panel::Destination) {
                app.mode = Mode::Normal;
            }
        }

        // Tab - path autocomplete
        KeyCode::Tab => {
            let current_path = match app.active_panel {
                Panel::Source => app.source.clone(),
                Panel::Destination => app.destination.clone(),
                _ => return,
            };

            if let Some(completed) = path::complete_path(&current_path) {
                match app.active_panel {
                    Panel::Source => {
                        app.source = completed.clone();
                        app.source_cursor = completed.len();
                    }
                    Panel::Destination => {
                        app.destination = completed.clone();
                        app.dest_cursor = completed.len();
                    }
                    _ => {}
                }
            }
        }

        // Cursor movement
        KeyCode::Left => {
            match app.active_panel {
                Panel::Source => {
                    if app.source_cursor > 0 {
                        app.source_cursor = prev_char_index(&app.source, app.source_cursor);
                    }
                }
                Panel::Destination => {
                    if app.dest_cursor > 0 {
                        app.dest_cursor = prev_char_index(&app.destination, app.dest_cursor);
                    }
                }
                _ => {}
            }
        }

        KeyCode::Right => {
            match app.active_panel {
                Panel::Source => {
                    if app.source_cursor < app.source.len() {
                        app.source_cursor = next_char_index(&app.source, app.source_cursor);
                    }
                }
                Panel::Destination => {
                    if app.dest_cursor < app.destination.len() {
                        app.dest_cursor = next_char_index(&app.destination, app.dest_cursor);
                    }
                }
                _ => {}
            }
        }

        KeyCode::Home => {
            match app.active_panel {
                Panel::Source => app.source_cursor = 0,
                Panel::Destination => app.dest_cursor = 0,
                _ => {}
            }
        }

        KeyCode::End => {
            match app.active_panel {
                Panel::Source => app.source_cursor = app.source.len(),
                Panel::Destination => app.dest_cursor = app.destination.len(),
                _ => {}
            }
        }

        // Delete at cursor
        KeyCode::Delete => {
            match app.active_panel {
                Panel::Source => {
                    if app.source_cursor < app.source.len() {
                        let next_idx = next_char_index(&app.source, app.source_cursor);
                        app.source.drain(app.source_cursor..next_idx);
                    }
                }
                Panel::Destination => {
                    if app.dest_cursor < app.destination.len() {
                        let next_idx = next_char_index(&app.destination, app.dest_cursor);
                        app.destination.drain(app.dest_cursor..next_idx);
                    }
                }
                _ => {}
            }
        }

        // Text input (allow Shift for uppercase)
        KeyCode::Char(c)
            if !key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
        {
            match app.active_panel {
                Panel::Source => {
                    app.source.insert(app.source_cursor, c);
                    app.source_cursor += c.len_utf8();
                }
                Panel::Destination => {
                    app.destination.insert(app.dest_cursor, c);
                    app.dest_cursor += c.len_utf8();
                }
                _ => {}
            }
        }

        // Backspace - delete before cursor
        KeyCode::Backspace => {
            match app.active_panel {
                Panel::Source => {
                    if app.source_cursor > 0 {
                        let prev_idx = prev_char_index(&app.source, app.source_cursor);
                        app.source.drain(prev_idx..app.source_cursor);
                        app.source_cursor = prev_idx;
                    }
                }
                Panel::Destination => {
                    if app.dest_cursor > 0 {
                        let prev_idx = prev_char_index(&app.destination, app.dest_cursor);
                        app.destination.drain(prev_idx..app.dest_cursor);
                        app.dest_cursor = prev_idx;
                    }
                }
                _ => {}
            }
        }

        _ => {}
    }
}

/// Index of the previous character boundary in a string before `byte_idx`
fn prev_char_index(s: &str, byte_idx: usize) -> usize {
    let mut idx = byte_idx;
    while idx > 0 {
        idx -= 1;
        if s.is_char_boundary(idx) {
            return idx;
        }
    }
    0
}

/// Index of the next character boundary in a string after `byte_idx`
fn next_char_index(s: &str, byte_idx: usize) -> usize {
    let mut idx = byte_idx + 1;
    while idx <= s.len() {
        if s.is_char_boundary(idx) {
            return idx;
        }
        idx += 1;
    }
    s.len()
}

/// Gate a run request: refuse invalid state, confirm destructive runs
fn run_rsync(app: &mut App, dry_run: bool) {
    if app.running {
        app.log("A transfer is already running".to_string());
        return;
    }
    if missing_paths(&app.source, &app.destination) {
        app.log("Set both source and destination before running".to_string());
        return;
    }

    let effective_dry = dry_run || app.options.dry_run;
    if !effective_dry && app.options.has_destructive() {
        app.confirm = Some(Confirm::Run { dry_run });
        return;
    }

    start_rsync(app, dry_run);
}

/// Spawn rsync in the background; output arrives via drain_rsync_events
fn start_rsync(app: &mut App, dry_run: bool) {
    let mut opts = app.options.clone();
    if dry_run {
        opts.dry_run = true;
    }

    let args = build_command(&app.source, &app.destination, &opts);
    app.log(format!("Running: {}", args.join(" ")));
    app.clear_progress();

    match runner::spawn(&args, opts.global_progress) {
        Ok(r) => {
            app.pending_cleanup = opts.should_cleanup_source();
            app.runner = Some(r);
            app.running = true;
        }
        Err(e) => app.log(format!("Failed to execute rsync: {}", e)),
    }
}

/// Apply pending runner events; reap the process once its output ends
fn drain_rsync_events(app: &mut App) {
    let mut runner = match app.runner.take() {
        Some(runner) => runner,
        None => return,
    };

    let mut finished = false;
    loop {
        match runner.events.try_recv() {
            Ok(RsyncEvent::Progress(percent, info)) => {
                app.progress_percentage = percent;
                app.transfer_info = info;
            }
            Ok(RsyncEvent::Line(line)) => app.log(line),
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => {
                finished = true;
                break;
            }
        }
    }

    if finished {
        let status = runner.child.wait().ok();
        on_rsync_finished(app, status);
    } else {
        app.runner = Some(runner);
    }
}

/// Handle rsync completion: report the outcome, run cleanup if due, save config
fn on_rsync_finished(app: &mut App, status: Option<std::process::ExitStatus>) {
    app.running = false;
    let success = status.map(|s| s.success()).unwrap_or(false);

    if success {
        app.progress_percentage = 100.0;
        app.log("Sync completed successfully".to_string());
        if app.pending_cleanup {
            cleanup_source_dirs(app);
        }
    } else {
        let code = status.and_then(|s| s.code());
        app.log(format!("Sync failed: {}", describe_exit_code(code)));
    }
    app.pending_cleanup = false;

    // Save config after every run
    let _ = Config {
        source: app.source.clone(),
        destination: app.destination.clone(),
        options: app.options.clone(),
    }
    .save();
}

/// Handle keys while a confirmation modal is open
fn handle_confirm_key(app: &mut App, key: &ratatui::crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => match app.confirm.take() {
            Some(Confirm::Run { dry_run }) => start_rsync(app, dry_run),
            Some(Confirm::Cancel) => cancel_rsync(app),
            None => {}
        },
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.confirm = None,
        _ => {}
    }
}

/// Kill a running transfer, if any
fn cancel_rsync(app: &mut App) {
    if let Some(runner) = app.runner.as_mut() {
        let _ = runner.child.kill();
        app.log("Transfer cancelled".to_string());
    }
}

/// Remove empty directories left in the source after --remove-source-files
fn cleanup_source_dirs(app: &mut App) {
    use std::process::Command;

    app.log("Cleaning up empty source directories...".to_string());
    let result = Command::new("find")
        .args([&app.source, "-type", "d", "-empty", "-delete"])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            app.log("Empty directories removed".to_string());
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            app.log(format!("Find command failed: {}", stderr));
        }
        Err(e) => app.log(format!("Failed to run find: {}", e)),
    }
}

/// True when either rsync path is empty
fn missing_paths(source: &str, destination: &str) -> bool {
    source.trim().is_empty() || destination.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_paths() {
        assert!(missing_paths("", "/dest"));
        assert!(missing_paths("/src", ""));
        assert!(missing_paths("   ", "/dest"));
        assert!(!missing_paths("/src", "/dest"));
    }

    #[test]
    fn test_run_rsync_refuses_empty_paths() {
        let mut app = App::new();
        run_rsync(&mut app, false);

        assert!(!app.running);
        assert!(app.logs.back().unwrap().contains("source and destination"));
    }

    #[test]
    fn test_run_rsync_refuses_when_already_running() {
        let mut app = App::new();
        app.running = true;
        app.source = "/src".to_string();
        app.destination = "/dest".to_string();
        run_rsync(&mut app, false);

        assert!(app.runner.is_none());
        assert!(app.logs.back().unwrap().contains("already running"));
    }

    #[test]
    fn test_run_rsync_destructive_requires_confirmation() {
        let mut app = App::new();
        app.source = "/src".to_string();
        app.destination = "/dest".to_string();
        app.options.delete = true;
        run_rsync(&mut app, false);

        assert!(!app.running);
        assert_eq!(app.confirm, Some(Confirm::Run { dry_run: false }));
    }

    #[test]
    fn test_confirm_dismiss_keeps_idle() {
        let mut app = App::new();
        app.confirm = Some(Confirm::Run { dry_run: false });
        handle_confirm_key(
            &mut app,
            &ratatui::crossterm::event::KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
        );

        assert!(app.confirm.is_none());
        assert!(!app.running);
    }

    #[test]
    fn test_confirm_cancel_clears_state() {
        let mut app = App::new();
        app.confirm = Some(Confirm::Cancel);
        handle_confirm_key(
            &mut app,
            &ratatui::crossterm::event::KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        );

        assert!(app.confirm.is_none());
    }
}
