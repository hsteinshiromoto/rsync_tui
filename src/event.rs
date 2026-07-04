use ratatui::crossterm::event::{self, Event, KeyEvent};
use std::time::Duration;

/// Poll for keyboard events with timeout
pub fn poll_event(timeout_ms: u64) -> anyhow::Result<Option<KeyEvent>> {
    if event::poll(Duration::from_millis(timeout_ms))? {
        if let Event::Key(key_event) = event::read()? {
            return Ok(Some(key_event));
        }
    }
    Ok(None)
}
