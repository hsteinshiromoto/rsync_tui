use ratatui::style::{Color, Modifier, Style};

// Tokyo Night base palette
pub const BLUE: Color = Color::Rgb(122, 162, 247);      // #7aa2f7 - Primary accent
pub const PURPLE: Color = Color::Rgb(187, 154, 247);    // #bb9af7 - Secondary accent
pub const CYAN: Color = Color::Rgb(125, 207, 255);      // #7dcfff - Highlights
pub const GREEN: Color = Color::Rgb(115, 218, 202);     // #73daca - Success
pub const RED: Color = Color::Rgb(247, 118, 142);       // #f7768e - Errors
pub const ORANGE: Color = Color::Rgb(255, 158, 100);    // #ff9e64 - Warnings
const BG_DARK: Color = Color::Rgb(26, 27, 38);          // #1a1b26 - Background
const TERMINAL_BLACK: Color = Color::Rgb(65, 72, 104);  // #414868 - Terminal black

// Semantic colors
pub const INACTIVE_BORDER: Color = TERMINAL_BLACK;
pub const TEXT_PRIMARY: Color = Color::Rgb(192, 202, 245);   // #c0caf5 - Terminal white
pub const TEXT_SECONDARY: Color = Color::Rgb(169, 177, 214); // #a9b1d6 - Foreground
pub const TEXT_PLACEHOLDER: Color = Color::Rgb(86, 95, 137); // #565f89 - Comments

// Progress bar
pub const PROGRESS_FILLED: Color = BLUE;
pub const PROGRESS_UNFILLED: Color = TERMINAL_BLACK;

// Option pills
pub const PILL_ENABLED_BG: Color = BLUE;
pub const PILL_ENABLED_FG: Color = BG_DARK;
pub const PILL_DISABLED_BG: Color = TERMINAL_BLACK;
pub const PILL_DISABLED_FG: Color = TEXT_SECONDARY;

// Help bar
pub const KEY_HINT_FG: Color = PURPLE;
pub const KEY_DESC_FG: Color = TEXT_SECONDARY;

// Mode badge
pub const MODE_NORMAL_BG: Color = GREEN;
pub const MODE_NORMAL_FG: Color = BG_DARK;
pub const MODE_INSERT_BG: Color = ORANGE;
pub const MODE_INSERT_FG: Color = BG_DARK;

pub fn border_style(active: bool) -> Style {
    if active {
        Style::default().fg(BLUE)
    } else {
        Style::default().fg(INACTIVE_BORDER)
    }
}

pub fn title_style(active: bool) -> Style {
    if active {
        Style::default().fg(PURPLE).add_modifier(Modifier::BOLD)
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

pub fn pill_danger() -> Style {
    Style::default()
        .fg(BG_DARK)
        .bg(RED)
        .add_modifier(Modifier::BOLD)
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
