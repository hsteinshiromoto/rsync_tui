use ratatui::style::{Color, Modifier, Style};

// Primary palette (Charm-inspired)
pub const PURPLE: Color = Color::Rgb(124, 58, 237);
pub const PINK: Color = Color::Rgb(236, 72, 153);
pub const LAVENDER: Color = Color::Rgb(167, 139, 250);

// Semantic colors
pub const INACTIVE_BORDER: Color = Color::Rgb(75, 85, 99);
pub const TEXT_PRIMARY: Color = Color::Rgb(229, 231, 235);
pub const TEXT_SECONDARY: Color = Color::Rgb(156, 163, 175);
pub const TEXT_PLACEHOLDER: Color = Color::Rgb(107, 114, 128);

// Status colors
pub const SUCCESS: Color = Color::Rgb(52, 211, 153);
pub const ERROR: Color = Color::Rgb(248, 113, 113);

// Progress bar
pub const PROGRESS_FILLED: Color = Color::Rgb(124, 58, 237);
pub const PROGRESS_UNFILLED: Color = Color::Rgb(55, 65, 81);

// Option pills
pub const PILL_ENABLED_BG: Color = Color::Rgb(124, 58, 237);
pub const PILL_ENABLED_FG: Color = Color::Rgb(255, 255, 255);
pub const PILL_DISABLED_BG: Color = Color::Rgb(55, 65, 81);
pub const PILL_DISABLED_FG: Color = Color::Rgb(156, 163, 175);

// Help bar
pub const KEY_HINT_FG: Color = Color::Rgb(167, 139, 250);
pub const KEY_DESC_FG: Color = Color::Rgb(156, 163, 175);

// Mode badge
pub const MODE_NORMAL_BG: Color = Color::Rgb(20, 184, 166);
pub const MODE_NORMAL_FG: Color = Color::Rgb(255, 255, 255);
pub const MODE_INSERT_BG: Color = Color::Rgb(236, 72, 153);
pub const MODE_INSERT_FG: Color = Color::Rgb(255, 255, 255);

pub fn border_style(active: bool) -> Style {
    if active {
        Style::default().fg(PURPLE)
    } else {
        Style::default().fg(INACTIVE_BORDER)
    }
}

pub fn title_style(active: bool) -> Style {
    if active {
        Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT_SECONDARY)
    }
}

pub fn text_primary() -> Style {
    Style::default().fg(TEXT_PRIMARY)
}

pub fn text_placeholder() -> Style {
    Style::default()
        .fg(TEXT_PLACEHOLDER)
        .add_modifier(Modifier::ITALIC)
}

pub fn pill_style(enabled: bool) -> Style {
    if enabled {
        Style::default()
            .fg(PILL_ENABLED_FG)
            .bg(PILL_ENABLED_BG)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(PILL_DISABLED_FG).bg(PILL_DISABLED_BG)
    }
}

pub fn key_hint() -> Style {
    Style::default()
        .fg(KEY_HINT_FG)
        .add_modifier(Modifier::BOLD)
}

pub fn key_desc() -> Style {
    Style::default().fg(KEY_DESC_FG)
}

pub fn gauge_filled() -> Style {
    Style::default().fg(PROGRESS_FILLED).bg(PROGRESS_UNFILLED)
}

pub fn mode_badge(is_normal: bool) -> Style {
    if is_normal {
        Style::default()
            .fg(MODE_NORMAL_FG)
            .bg(MODE_NORMAL_BG)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(MODE_INSERT_FG)
            .bg(MODE_INSERT_BG)
            .add_modifier(Modifier::BOLD)
    }
}
