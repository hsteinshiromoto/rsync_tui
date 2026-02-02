mod app;
mod event;
mod rsync;
mod ui;

use std::io;
use app::{App, Panel};
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
            // Global quit
            if event::is_quit(&key) {
                app.should_quit = true;
            }

            match key.code {
                // Panel navigation
                KeyCode::Tab => app.next_panel(),
                KeyCode::BackTab => app.prev_panel(),

                // Sync commands (Ctrl+key)
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    run_rsync(app, false);
                }
                KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    run_rsync(app, true);
                }

                // Text input for source/destination (allow Shift for uppercase)
                KeyCode::Char(c)
                    if matches!(app.active_panel, Panel::Source | Panel::Destination)
                        && !key.modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
                {
                    match app.active_panel {
                        Panel::Source => app.source.push(c),
                        Panel::Destination => app.destination.push(c),
                        _ => {}
                    }
                }

                // Option toggles (only when NOT in text input panels)
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    if let Some(idx) = c.to_digit(10) {
                        if idx >= 1 && idx <= 8 {
                            app.options.toggle((idx - 1) as usize);
                        }
                    }
                }

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

        if app.should_quit {
            break;
        }
    }

    Ok(())
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
