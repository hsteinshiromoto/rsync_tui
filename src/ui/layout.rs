use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Panel};
use crate::rsync::command::format_command;

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // Source (100% width)
            Constraint::Length(3),  // Destination (100% width)
            Constraint::Length(5),  // Options
            Constraint::Min(5),     // Logs
            Constraint::Length(3),  // Help bar
        ])
        .split(frame.size());

    render_title(frame, chunks[0]);
    render_source(frame, chunks[1], app);
    render_destination(frame, chunks[2], app);
    render_options(frame, chunks[3], app);
    render_logs(frame, chunks[4], app);
    render_help(frame, chunks[5]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("rsync TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
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
            .title("Source")
            .borders(Borders::ALL)
            .border_style(style),
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
            .title("Destination")
            .borders(Borders::ALL)
            .border_style(style),
    );
    frame.render_widget(dest, area);
}

fn render_options(frame: &mut Frame, area: Rect, app: &App) {
    let opts = &app.options;
    let items = vec![
        format_option("1", "Archive", opts.archive),
        format_option("2", "Verbose", opts.verbose),
        format_option("3", "Compress", opts.compress),
        format_option("4", "Dry-run", opts.dry_run),
        format_option("5", "Progress", opts.progress),
        format_option("6", "Delete", opts.delete),
        format_option("7", "Human", opts.human_readable),
        format_option("8", "SSH", opts.use_ssh),
    ];

    let options_text = items.join("  ");
    let style = panel_style(app.active_panel == Panel::Options);

    let options = Paragraph::new(options_text).block(
        Block::default()
            .title("Options (press number to toggle)")
            .borders(Borders::ALL)
            .border_style(style),
    );
    frame.render_widget(options, area);
}

fn render_logs(frame: &mut Frame, area: Rect, app: &App) {
    let style = panel_style(app.active_panel == Panel::Logs);

    // Show command preview at top, then logs
    let cmd = format_command(&app.source, &app.destination, &app.options);
    let mut lines: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Green)),
            Span::raw(cmd),
        ])),
        ListItem::new(""),
    ];

    // Add log entries
    for log in app.logs.iter().rev().take(20) {
        lines.push(ListItem::new(log.as_str()));
    }

    let logs = List::new(lines).block(
        Block::default()
            .title("Preview / Logs")
            .borders(Borders::ALL)
            .border_style(style),
    );
    frame.render_widget(logs, area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_text = "[Tab] Switch panel  [Ctrl+s] Sync  [Ctrl+n] Dry-run  [1-8] Toggle options  [q] Quit";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
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
