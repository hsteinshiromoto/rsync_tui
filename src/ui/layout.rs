use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Padding, Paragraph},
    Frame,
};

use crate::app::{App, Mode, Panel};
use crate::rsync::command::format_command;

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Source (100% width)
            Constraint::Length(3),  // Destination (100% width)
            Constraint::Length(6),  // Options (2 rows)
            Constraint::Length(7),  // Logs (multiline command support)
            Constraint::Min(6),     // Progress
            Constraint::Length(3),  // Help bar
        ])
        .split(frame.size());

    render_title(frame, chunks[0], app);
    render_source(frame, chunks[1], app);
    render_destination(frame, chunks[2], app);
    render_options(frame, chunks[3], app);
    render_logs(frame, chunks[4], app);
    render_progress(frame, chunks[5], app);
    render_help(frame, chunks[6], app);
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let mode_str = match app.mode {
        Mode::Normal => "[NORMAL]",
        Mode::Insert => "[INSERT]",
    };
    let mode_color = match app.mode {
        Mode::Normal => Color::Green,
        Mode::Insert => Color::Yellow,
    };

    let title = Paragraph::new(Line::from(vec![
        Span::styled("rsync TUI ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(mode_str, Style::default().fg(mode_color).add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::ALL).padding(Padding::horizontal(1)));
    frame.render_widget(title, area);
}

fn render_source(frame: &mut Frame, area: Rect, app: &App) {
    let style = panel_style(app.active_panel == Panel::Source);
    let source = Paragraph::new(if app.source.is_empty() {
        "<enter source path>".to_string()
    } else {
        app.source.clone()
    })
    .style(if app.source.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    })
    .block(
        Block::default()
            .title("[1] Source")
            .borders(Borders::ALL)
            .border_style(style)
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(source, area);
}

fn render_destination(frame: &mut Frame, area: Rect, app: &App) {
    let style = panel_style(app.active_panel == Panel::Destination);
    let dest = Paragraph::new(if app.destination.is_empty() {
        "<enter destination path>".to_string()
    } else {
        app.destination.clone()
    })
    .style(if app.destination.is_empty() {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    })
    .block(
        Block::default()
            .title("[2] Destination")
            .borders(Borders::ALL)
            .border_style(style)
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(dest, area);
}

fn render_options(frame: &mut Frame, area: Rect, app: &App) {
    let opts = &app.options;
    let row1 = vec![
        format_option("a", "Archive", opts.archive),
        format_option("v", "Verbose", opts.verbose),
        format_option("z", "Compress", opts.compress),
        format_option("n", "Dry-run", opts.dry_run),
        format_option("p", "Progress/file", opts.progress),
        format_option("d", "Delete", opts.delete),
    ];
    let row2 = vec![
        format_option("h", "Human", opts.human_readable),
        format_option("e", "SSH", opts.use_ssh),
        format_option("r", "DelSrc", opts.delete_source),
        format_option("x", "DelExcl", opts.delete_excluded),
        format_option("f", "GlobalProgress", opts.progress_per_file),
    ];

    let options_text = format!("{}\n{}", row1.join("  "), row2.join("  "));
    let style = panel_style(app.active_panel == Panel::Options);

    let options = Paragraph::new(options_text).block(
        Block::default()
            .title("[3] Options")
            .borders(Borders::ALL)
            .border_style(style)
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(options, area);
}

fn render_logs(frame: &mut Frame, area: Rect, app: &App) {
    let style = panel_style(app.active_panel == Panel::Logs);

    // Calculate available width: area - borders(2) - padding(2) - prefix(2)
    let inner_width = area.width.saturating_sub(6) as usize;

    // Show command preview at top, then logs
    let cmd = format_command(&app.source, &app.destination, &app.options);
    let mut lines: Vec<ListItem> = Vec::new();

    // Wrap command lines to fit within panel width
    let wrapped_lines = wrap_command(&cmd, inner_width);
    for (i, line) in wrapped_lines.iter().enumerate() {
        if i == 0 {
            lines.push(ListItem::new(Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Green)),
                Span::raw(line.clone()),
            ])));
        } else {
            lines.push(ListItem::new(Line::from(vec![
                Span::styled("  ", Style::default().fg(Color::Green)),
                Span::raw(line.clone()),
            ])));
        }
    }
    lines.push(ListItem::new(""));

    // Add log entries
    for log in app.logs.iter().rev().take(20) {
        lines.push(ListItem::new(log.as_str()));
    }

    let logs = List::new(lines).block(
        Block::default()
            .title("[4] Preview / Logs")
            .borders(Borders::ALL)
            .border_style(style)
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(logs, area);
}

/// Wrap command string to fit within given width, using \ for continuation
fn wrap_command(cmd: &str, max_width: usize) -> Vec<String> {
    let mut result = Vec::new();

    for line in cmd.split('\n') {
        let line = line.trim_start();
        if line.len() <= max_width {
            result.push(line.to_string());
        } else {
            // Wrap long lines
            let mut remaining = line;
            let mut is_first = true;
            while !remaining.is_empty() {
                // Reserve space for " \" continuation on non-final segments
                let wrap_at = if remaining.len() > max_width {
                    max_width.saturating_sub(2)
                } else {
                    remaining.len()
                };

                // Try to break at a space
                let break_pos = if wrap_at < remaining.len() {
                    remaining[..wrap_at]
                        .rfind(' ')
                        .map(|p| p + 1)
                        .unwrap_or(wrap_at)
                } else {
                    wrap_at
                };

                let (chunk, rest) = remaining.split_at(break_pos);
                let chunk = chunk.trim_end();

                if rest.is_empty() || rest.trim().is_empty() {
                    result.push(if is_first { chunk.to_string() } else { format!("  {}", chunk) });
                } else {
                    result.push(if is_first { format!("{} \\", chunk) } else { format!("  {} \\", chunk) });
                }

                remaining = rest.trim_start();
                is_first = false;
            }
        }
    }

    result
}

fn render_progress(frame: &mut Frame, area: Rect, app: &App) {
    let style = panel_style(app.active_panel == Panel::Progress);

    // Split area: top for gauge, bottom for output
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Progress bar
            Constraint::Min(1),    // Output lines
        ])
        .split(area);

    // Progress bar with percentage
    let label = if app.transfer_info.is_empty() {
        format!("{:.0}%", app.progress_percentage)
    } else {
        format!("{:.0}% - {}", app.progress_percentage, app.transfer_info)
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("[5] Progress")
                .borders(Borders::ALL)
                .border_style(style)
                .padding(Padding::horizontal(1)),
        )
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .percent(app.progress_percentage as u16)
        .label(label);
    frame.render_widget(gauge, inner_chunks[0]);

    // Rsync output lines
    let output_lines: Vec<ListItem> = app
        .progress_output
        .iter()
        .rev()
        .take(10)
        .map(|line| ListItem::new(line.as_str()))
        .collect();

    let output = List::new(output_lines).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_style(style)
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(output, inner_chunks[1]);
}

fn render_help(frame: &mut Frame, area: Rect, app: &App) {
    let help_text = match (&app.mode, &app.active_panel) {
        (Mode::Normal, Panel::Logs) => "[1-5/j/k] Panels  [Enter] Run  [i] Insert  [a/v/z/n/p/d/h/e/r/x/f] Options  [q] Quit",
        (Mode::Normal, _) => "[1-5/j/k] Panels  [i] Insert  [a/v/z/n/p/d/h/e/r/x/f] Options  [Ctrl+s] Sync  [q] Quit",
        (Mode::Insert, _) => "[Esc] Normal  [Enter] Next  [Tab] Autocomplete  [Ctrl+s] Sync  [Ctrl+n] Dry-run",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).padding(Padding::horizontal(1)));
    frame.render_widget(help, area);
}

fn panel_style(active: bool) -> Style {
    if active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    }
}

fn format_option(key: &str, name: &str, enabled: bool) -> String {
    let check = if enabled { "x" } else { " " };
    format!("[{}]{} {}", check, key, name)
}
