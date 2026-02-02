use crate::rsync::options::RsyncOptions;

/// Active panel in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Source,
    Destination,
    Options,
    Logs,
}

/// Vim-like editing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

/// Application state
pub struct App {
    pub source: String,
    pub destination: String,
    pub options: RsyncOptions,
    pub logs: Vec<String>,
    pub active_panel: Panel,
    pub mode: Mode,
    #[allow(dead_code)] // Reserved for future async progress tracking
    pub running: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            destination: String::new(),
            options: RsyncOptions::default(),
            logs: Vec::new(),
            active_panel: Panel::Source,
            mode: Mode::Normal,
            running: false,
            should_quit: false,
        }
    }

    /// Move focus to next panel
    pub fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Source => Panel::Destination,
            Panel::Destination => Panel::Options,
            Panel::Options => Panel::Logs,
            Panel::Logs => Panel::Source,
        };
    }

    /// Move focus to previous panel
    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Source => Panel::Logs,
            Panel::Destination => Panel::Source,
            Panel::Options => Panel::Destination,
            Panel::Logs => Panel::Options,
        };
    }

    /// Add a log message
    pub fn log(&mut self, message: String) {
        self.logs.push(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new_defaults() {
        let app = App::new();

        assert!(app.source.is_empty());
        assert!(app.destination.is_empty());
        assert!(app.logs.is_empty());
        assert_eq!(app.active_panel, Panel::Source);
        assert_eq!(app.mode, Mode::Normal);
        assert!(!app.running);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_next_panel_cycles_forward() {
        let mut app = App::new();

        assert_eq!(app.active_panel, Panel::Source);
        app.next_panel();
        assert_eq!(app.active_panel, Panel::Destination);
        app.next_panel();
        assert_eq!(app.active_panel, Panel::Options);
        app.next_panel();
        assert_eq!(app.active_panel, Panel::Logs);
        app.next_panel();
        assert_eq!(app.active_panel, Panel::Source); // Wraps around
    }

    #[test]
    fn test_prev_panel_cycles_backward() {
        let mut app = App::new();

        assert_eq!(app.active_panel, Panel::Source);
        app.prev_panel();
        assert_eq!(app.active_panel, Panel::Logs); // Wraps around
        app.prev_panel();
        assert_eq!(app.active_panel, Panel::Options);
        app.prev_panel();
        assert_eq!(app.active_panel, Panel::Destination);
        app.prev_panel();
        assert_eq!(app.active_panel, Panel::Source);
    }

    #[test]
    fn test_log_adds_message() {
        let mut app = App::new();

        assert!(app.logs.is_empty());
        app.log("First message".to_string());
        assert_eq!(app.logs.len(), 1);
        assert_eq!(app.logs[0], "First message");

        app.log("Second message".to_string());
        assert_eq!(app.logs.len(), 2);
        assert_eq!(app.logs[1], "Second message");
    }
}
