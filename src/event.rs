use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
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

/// Check if key is quit command (q or Ctrl+C)
pub fn is_quit(key: &KeyEvent) -> bool {
    matches!(
        key,
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        } | KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }
    )
}
