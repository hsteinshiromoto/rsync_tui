use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use super::theme;
use crate::app::{App, Confirm, Mode, Panel};
use crate::rsync::command::format_command;
use crate::rsync::options::{OptionDef, OPTIONS};

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
    render_path_input(frame, chunks[1], &app.source, "1", "Source", app.active_panel == Panel::Source);
    render_path_input(frame, chunks[2], &app.destination, "2", "Destination", app.active_panel == Panel::Destination);
    render_options(frame, chunks[3], app);
    render_logs(frame, chunks[4], app);
    render_progress(frame, chunks[5], app);
    render_help(frame, chunks[6], app);

    if let Some(confirm) = app.confirm {
        render_confirm_modal(frame, app, confirm);
    }
}

/// Centred overlay area clamped to the frame
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 2,
        width: w,
        height: h,
    }
}

fn render_confirm_modal(frame: &mut Frame, app: &App, confirm: Confirm) {
    let message = match confirm {
        Confirm::Run { .. } => {
            let flags: Vec<&str> = OPTIONS
                .iter()
                .filter(|def| def.destructive && (def.get)(&app.options))
                .map(|def| def.flag)
                .collect();
            format!("This run can delete files ({}). Proceed?", flags.join(", "))
        }
        Confirm::Cancel => "Stop the running transfer?".to_string(),
    };

    let area = centered_rect(60, 6, frame.size());
    frame.render_widget(Clear, area);
    let modal = Paragraph::new(vec![
        Line::from(Span::styled(message, theme::text_primary())),
        Line::from(""),
        Line::from(vec![
            Span::styled(" y/Enter ", theme::key_hint()),
            Span::styled(" confirm   ", theme::key_desc()),
            Span::styled(" n/Esc ", theme::key_hint()),
            Span::styled(" dismiss", theme::key_desc()),
        ]),
    ])
    .wrap(Wrap { trim: true })
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Span::styled(
                " Confirm ",
                Style::default().fg(theme::RED).add_modifier(Modifier::BOLD),
            ))
            .border_style(Style::default().fg(theme::RED))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(modal, area);
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let (mode_str, is_normal) = match app.mode {
        Mode::Normal => (" NORMAL ", true),
        Mode::Insert => (" INSERT ", false),
    };

    let mut title_spans = vec![
        Span::styled(
            " rsync TUI ",
            Style::default().fg(theme::PURPLE).add_modifier(Modifier::BOLD),
        ),
        Span::styled(mode_str, theme::mode_badge(is_normal)),
    ];
    if app.running {
        title_spans.push(Span::styled(
            "  RUNNING (Esc cancels)",
            Style::default().fg(theme::ORANGE).add_modifier(Modifier::BOLD),
        ));
    }

    let title = Paragraph::new(Line::from(title_spans)).block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BLUE))
            .padding(Padding::horizontal(1)),
    );
    frame.render_widget(title, area);
}

fn render_path_input(
    frame: &mut Frame,
    area: Rect,
    value: &str,
    badge: &str,
    label: &str,
    active: bool,
) {
    let (display_text, content_style) = if value.is_empty() {
        (
            format!("enter {} path...", label.to_lowercase()),
            theme::text_placeholder(),
        )
    } else {
        (value.to_string(), theme::text_primary())
    };

    let title_line = Line::from(vec![
        Span::styled(format!(" {} ", badge), theme::pill_style(active)),
        Span::styled(format!(" {} ", label), theme::title_style(active)),
    ]);

    let input = Paragraph::new(display_text)
        .style(content_style)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(title_line)
                .border_style(theme::border_style(active))
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(input, area);
}

fn render_options(frame: &mut Frame, area: Rect, app: &App) {
    let opts = &app.options;
    let active = app.active_panel == Panel::Options;

    let mut row1_spans: Vec<Span> = Vec::new();
    let mut row2_spans: Vec<Span> = Vec::new();
    for (i, def) in OPTIONS.iter().enumerate() {
        let row = if i < 6 { &mut row1_spans } else { &mut row2_spans };
        row.extend(format_option_pill(def, (def.get)(opts)));
    }

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

/// Wrap command string to fit within given width, using \ for continuation.
/// Widths are counted in characters and split points kept on char
/// boundaries, so multi-byte paths cannot cause a panic.
fn wrap_command(cmd: &str, max_width: usize) -> Vec<String> {
    let mut result = Vec::new();

    for line in cmd.split('\n') {
        let line = line.trim_start();
        if line.chars().count() <= max_width {
            result.push(line.to_string());
        } else {
            let mut remaining = line;
            let mut is_first = true;
            while !remaining.is_empty() {
                let char_count = remaining.chars().count();
                // At least one char per line so the loop always advances
                let wrap_chars = if char_count > max_width {
                    max_width.saturating_sub(2).max(1)
                } else {
                    char_count
                };
                // Byte index of the wrap point (always a char boundary)
                let wrap_at = remaining
                    .char_indices()
                    .nth(wrap_chars)
                    .map(|(byte, _)| byte)
                    .unwrap_or(remaining.len());

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
        .logs
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
    let option_keys: String = OPTIONS
        .iter()
        .map(|def| def.key.to_string())
        .collect::<Vec<_>>()
        .join("/");

    let pairs: Vec<(&str, &str)> = match (&app.mode, &app.active_panel) {
        (Mode::Normal, Panel::Logs) => vec![
            ("1-5/j/k", "Panels"),
            ("Enter", "Run"),
            ("i", "Insert"),
            (option_keys.as_str(), "Options"),
            ("q", "Quit"),
        ],
        (Mode::Normal, _) => vec![
            ("1-5/j/k", "Panels"),
            ("i", "Insert"),
            (option_keys.as_str(), "Options"),
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

fn format_option_pill<'a>(def: &'a OptionDef, enabled: bool) -> Vec<Span<'a>> {
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
    // Destructive options stand out in red when armed
    let label_style = if enabled && def.destructive {
        theme::pill_danger()
    } else {
        theme::pill_style(enabled)
    };

    vec![
        Span::styled(format!(" {} ", def.key), key_style),
        Span::styled(format!(" {} ", def.label), label_style),
        Span::raw(" "),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_command_short_line_unchanged() {
        let lines = wrap_command("rsync -a /src /dest", 40);

        assert_eq!(lines, vec!["rsync -a /src /dest"]);
    }

    #[test]
    fn test_wrap_command_long_line_continues_with_backslash() {
        let lines = wrap_command("rsync -a --progress /some/long/path /other/long/path", 30);

        assert!(lines.len() > 1);
        assert!(lines[0].ends_with('\\'));
    }

    #[test]
    fn test_wrap_command_multibyte_does_not_panic() {
        let cmd = "rsync -a /Übungsdaten/文件夹/José/längere/пути/ещё/长路径/mehr/Pfade /Bäckup/Zïel";
        for width in 1..60 {
            let lines = wrap_command(cmd, width);
            assert!(!lines.is_empty());
        }
    }
}
