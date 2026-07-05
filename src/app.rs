use std::collections::VecDeque;

use crate::rsync::options::RsyncOptions;
use crate::rsync::runner::RsyncRunner;

/// Maximum number of log lines kept in memory
const MAX_LOG_LINES: usize = 1000;

/// Active panel in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Source,
    Destination,
    Options,
    Logs,
    Progress,
}

/// Vim-like editing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
}

/// Pending action awaiting user confirmation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confirm {
    /// Start a run whose enabled options can destroy data
    Run { dry_run: bool },
    /// Stop the transfer currently running
    Cancel,
}

/// Application state
pub struct App {
    pub source: String,
    pub source_cursor: usize,
    pub destination: String,
    pub dest_cursor: usize,
    pub options: RsyncOptions,
    pub logs: VecDeque<String>,
    pub active_panel: Panel,
    pub mode: Mode,
    pub running: bool,
    pub should_quit: bool,
    pub confirm: Option<Confirm>,
    // Progress tracking
    pub runner: Option<RsyncRunner>,
    pub pending_cleanup: bool,
    pub progress_percentage: f64,
    pub transfer_info: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            source_cursor: 0,
            destination: String::new(),
            dest_cursor: 0,
            options: RsyncOptions::default(),
            logs: VecDeque::new(),
            active_panel: Panel::Source,
            mode: Mode::Normal,
            running: false,
            should_quit: false,
            confirm: None,
            runner: None,
            pending_cleanup: false,
            progress_percentage: 0.0,
            transfer_info: String::new(),
        }
    }

    /// Move focus to next panel
    pub fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Source => Panel::Destination,
            Panel::Destination => Panel::Options,
            Panel::Options => Panel::Logs,
            Panel::Logs => Panel::Progress,
            Panel::Progress => Panel::Source,
        };
    }

    /// Move focus to previous panel
    pub fn prev_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Source => Panel::Progress,
            Panel::Destination => Panel::Source,
            Panel::Options => Panel::Destination,
            Panel::Logs => Panel::Options,
            Panel::Progress => Panel::Logs,
        };
    }

    /// Clear progress state for new transfer
    pub fn clear_progress(&mut self) {
        self.progress_percentage = 0.0;
        self.transfer_info.clear();
    }

    /// Add a log message, keeping at most MAX_LOG_LINES entries
    pub fn log(&mut self, message: String) {
        self.logs.push_back(message);
        if self.logs.len() > MAX_LOG_LINES {
            self.logs.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new_defaults() {
        let app = App::new();

        assert!(app.source.is_empty());
        assert_eq!(app.source_cursor, 0);
        assert!(app.destination.is_empty());
        assert_eq!(app.dest_cursor, 0);
        assert!(app.logs.is_empty());
        assert_eq!(app.active_panel, Panel::Source);
        assert_eq!(app.mode, Mode::Normal);
        assert!(!app.running);
        assert!(!app.should_quit);
        assert!(app.confirm.is_none());
        assert!(app.runner.is_none());
        assert!(!app.pending_cleanup);
        assert_eq!(app.progress_percentage, 0.0);
        assert!(app.transfer_info.is_empty());
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
        assert_eq!(app.active_panel, Panel::Progress);
        app.next_panel();
        assert_eq!(app.active_panel, Panel::Source); // Wraps around
    }

    #[test]
    fn test_prev_panel_cycles_backward() {
        let mut app = App::new();

        assert_eq!(app.active_panel, Panel::Source);
        app.prev_panel();
        assert_eq!(app.active_panel, Panel::Progress); // Wraps around
        app.prev_panel();
        assert_eq!(app.active_panel, Panel::Logs);
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

    #[test]
    fn test_log_caps_at_max_lines() {
        let mut app = App::new();

        for i in 0..1100 {
            app.log(format!("line {}", i));
        }
        assert_eq!(app.logs.len(), 1000);
        assert_eq!(app.logs.front().unwrap(), "line 100");
    }
}
