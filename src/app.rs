use crate::rsync::options::RsyncOptions;

/// Active panel in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Source,
    Destination,
    Options,
    Logs,
}

/// Application state
pub struct App {
    pub source: String,
    pub destination: String,
    pub options: RsyncOptions,
    pub logs: Vec<String>,
    pub active_panel: Panel,
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
