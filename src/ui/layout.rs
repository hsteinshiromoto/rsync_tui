use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Padding, Paragraph},
    Frame,
};

use super::theme;
use crate::app::{App, Mode, Panel};
use crate::rsync::command::format_command;

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Source
            Constraint::Length(3), // Destination
            Constraint::Length(6), // Options (2 rows)
            Constraint::Length(7), // Logs
            Constraint::Min(6),   // Progress
            Constraint::Length(3), // Help bar
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
    let (mode_str, is_normal) = match app.mode {
        Mode::Normal => (" NORMAL ", true),
        Mode::Insert => (" INSERT ", false),
    };

    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            " rsync TUI ",
            Style::default().fg(theme::PURPLE).add_modifier(Modifier::BOLD),
        ),
        Span::styled(mode_str, theme::mode_badge(is_normal)),
    ]))
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BLUE))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(title, area);
}

fn render_source(frame: &mut Frame, area: Rect, app: &App) {
    let active = app.active_panel == Panel::Source;
    let (display_text, content_style) = if app.source.is_empty() {
        ("enter source path...".to_string(), theme::text_placeholder())
    } else {
        (app.source.clone(), theme::text_primary())
    };

    let title_line = Line::from(vec![
        Span::styled(" 1 ", theme::pill_style(active)),
        Span::styled(" Source ", theme::title_style(active)),
    ]);

    let source = Paragraph::new(display_text)
        .style(content_style)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title_line)
                .border_style(theme::border_style(active))
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(source, area);
}

fn render_destination(frame: &mut Frame, area: Rect, app: &App) {
    let active = app.active_panel == Panel::Destination;
    let (display_text, content_style) = if app.destination.is_empty() {
        ("enter destination path...".to_string(), theme::text_placeholder())
    } else {
        (app.destination.clone(), theme::text_primary())
    };

    let title_line = Line::from(vec![
        Span::styled(" 2 ", theme::pill_style(active)),
        Span::styled(" Destination ", theme::title_style(active)),
    ]);

    let dest = Paragraph::new(display_text)
        .style(content_style)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title_line)
                .border_style(theme::border_style(active))
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(dest, area);
}

fn render_options(frame: &mut Frame, area: Rect, app: &App) {
    let opts = &app.options;
    let active = app.active_panel == Panel::Options;

    let mut row1_spans: Vec<Span> = Vec::new();
    row1_spans.extend(format_option_pill("a", "Archive", opts.archive));
    row1_spans.extend(format_option_pill("v", "Verbose", opts.verbose));
    row1_spans.extend(format_option_pill("z", "Compress", opts.compress));
    row1_spans.extend(format_option_pill("n", "Dry-run", opts.dry_run));
    row1_spans.extend(format_option_pill("p", "Progress", opts.progress));
    row1_spans.extend(format_option_pill("d", "Delete", opts.delete));

    let mut row2_spans: Vec<Span> = Vec::new();
    row2_spans.extend(format_option_pill("h", "Human", opts.human_readable));
    row2_spans.extend(format_option_pill("e", "SSH", opts.use_ssh));
    row2_spans.extend(format_option_pill("r", "DelSrc", opts.delete_source));
    row2_spans.extend(format_option_pill("f", "Global", opts.progress_per_file));

    let title_line = Line::from(vec![
        Span::styled(" 3 ", theme::pill_style(active)),
        Span::styled(" Options ", theme::title_style(active)),
    ]);

    let text = ratatui::text::Text::from(vec![
        Line::from(row1_spans),
        Line::from(vec![Span::raw("")]),
        Line::from(row2_spans),
    ]);

    let options = Paragraph::new(text).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title_line)
            .border_style(theme::border_style(active))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(options, area);
}

fn render_logs(frame: &mut Frame, area: Rect, app: &App) {
    let active = app.active_panel == Panel::Logs;
    let inner_width = area.width.saturating_sub(6) as usize;

    let cmd = format_command(&app.source, &app.destination, &app.options);
    let mut lines: Vec<ListItem> = Vec::new();

    let wrapped_lines = wrap_command(&cmd, inner_width);
    for (i, line) in wrapped_lines.iter().enumerate() {
        let prefix = if i == 0 { "> " } else { "  " };
        lines.push(ListItem::new(Line::from(vec![
            Span::styled(prefix, Style::default().fg(theme::GREEN)),
            Span::styled(line.clone(), theme::text_primary()),
        ])));
    }
    lines.push(ListItem::new(""));

    for log in app.logs.iter().rev().take(20) {
        let style = if log.starts_with("[ERR]") {
            Style::default().fg(theme::RED)
        } else {
            Style::default().fg(theme::TEXT_SECONDARY)
        };
        lines.push(ListItem::new(Span::styled(log.as_str(), style)));
    }

    let title_line = Line::from(vec![
        Span::styled(" 4 ", theme::pill_style(active)),
        Span::styled(" Preview / Logs ", theme::title_style(active)),
    ]);

    let logs = List::new(lines).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title_line)
            .border_style(theme::border_style(active))
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
            let mut remaining = line;
            let mut is_first = true;
            while !remaining.is_empty() {
                let wrap_at = if remaining.len() > max_width {
                    max_width.saturating_sub(2)
                } else {
                    remaining.len()
                };

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
                    result.push(if is_first {
                        chunk.to_string()
                    } else {
                        format!("  {}", chunk)
                    });
                } else {
                    result.push(if is_first {
                        format!("{} \\", chunk)
                    } else {
                        format!("  {} \\", chunk)
                    });
                }

                remaining = rest.trim_start();
                is_first = false;
            }
        }
    }

    result
}

fn render_progress(frame: &mut Frame, area: Rect, app: &App) {
    let active = app.active_panel == Panel::Progress;

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Progress bar
            Constraint::Min(1),   // Output lines
        ])
        .split(area);

    let label = if app.transfer_info.is_empty() {
        format!("{:.0}%", app.progress_percentage)
    } else {
        format!("{:.0}% - {}", app.progress_percentage, app.transfer_info)
    };

    let title_line = Line::from(vec![
        Span::styled(" 5 ", theme::pill_style(active)),
        Span::styled(" Progress ", theme::title_style(active)),
    ]);

    let gauge = Gauge::default()
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title_line)
                .border_style(theme::border_style(active))
                .padding(Padding::horizontal(1)),
        )
        .gauge_style(theme::gauge_filled())
        .percent(app.progress_percentage as u16)
        .label(Span::styled(label, theme::text_primary()));
    frame.render_widget(gauge, inner_chunks[0]);

    let output_lines: Vec<ListItem> = app
        .progress_output
        .iter()
        .rev()
        .take(10)
        .map(|line| {
            let style = if line.starts_with("[ERR]") {
                Style::default().fg(theme::RED)
            } else {
                Style::default().fg(theme::TEXT_SECONDARY)
            };
            ListItem::new(Span::styled(line.as_str(), style))
        })
        .collect();

    let output = List::new(output_lines).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_type(BorderType::Rounded)
            .border_style(theme::border_style(active))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(output, inner_chunks[1]);
}

fn render_help(frame: &mut Frame, area: Rect, app: &App) {
    let pairs: Vec<(&str, &str)> = match (&app.mode, &app.active_panel) {
        (Mode::Normal, Panel::Logs) => vec![
            ("1-5/j/k", "Panels"),
            ("Enter", "Run"),
            ("i", "Insert"),
            ("a/v/z/n/p/d/h/e/r/f", "Options"),
            ("q", "Quit"),
        ],
        (Mode::Normal, _) => vec![
            ("1-5/j/k", "Panels"),
            ("i", "Insert"),
            ("a/v/z/n/p/d/h/e/r/f", "Options"),
            ("Ctrl+s", "Sync"),
            ("q", "Quit"),
        ],
        (Mode::Insert, _) => vec![
            ("Esc", "Normal"),
            ("Enter", "Next"),
            ("Tab", "Complete"),
            ("Ctrl+s", "Sync"),
            ("Ctrl+n", "Dry-run"),
        ],
    };

    let mut spans: Vec<Span> = Vec::new();
    for (i, (key, desc)) in pairs.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", theme::key_desc()));
        }
        spans.push(Span::styled(format!(" {} ", key), theme::key_hint()));
        spans.push(Span::styled(format!(" {}", desc), theme::key_desc()));
    }

    let help = Paragraph::new(Line::from(spans)).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::INACTIVE_BORDER))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(help, area);
}

fn format_option_pill<'a>(key: &'a str, name: &'a str, enabled: bool) -> Vec<Span<'a>> {
    let key_style = if enabled {
        Style::default()
            .fg(theme::PILL_ENABLED_FG)
            .bg(theme::CYAN)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(theme::KEY_HINT_FG)
            .bg(theme::PILL_DISABLED_BG)
            .add_modifier(Modifier::BOLD)
    };

    vec![
        Span::styled(format!(" {} ", key), key_style),
        Span::styled(format!(" {} ", name), theme::pill_style(enabled)),
        Span::raw(" "),
    ]
}
