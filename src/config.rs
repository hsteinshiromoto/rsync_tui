use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::rsync::options::RsyncOptions;

/// User configuration persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub source: String,
    pub destination: String,
    pub options: RsyncOptions,
}

impl Config {
    /// Load config from ~/.config/rsync_tui/config.json
    /// Returns None if the file doesn't exist or can't be parsed; errors are silent.
    pub fn load() -> Option<Self> {
        let path = config_path()?;
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }

    /// Save config to ~/.config/rsync_tui/config.json
    /// Creates the directory if needed; errors are silent.
    pub fn save(&self) -> bool {
        let path = match config_path() {
            Some(p) => p,
            None => return false,
        };

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let json = match serde_json::to_string_pretty(self) {
            Ok(j) => j,
            Err(_) => return false,
        };

        std::fs::write(&path, json).is_ok()
    }
}

/// Path to ~/.config/rsync_tui/config.json
fn config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(
        PathBuf::from(home)
            .join(".config")
            .join("rsync_tui")
            .join("config.json"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_roundtrip() {
        let cfg = Config {
            source: "/src".to_string(),
            destination: "/dest".to_string(),
            options: RsyncOptions::default(),
        };

        let json = serde_json::to_string(&cfg).unwrap();
        let decoded: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.source, "/src");
        assert_eq!(decoded.destination, "/dest");
        assert_eq!(decoded.options.archive, cfg.options.archive);
    }

    #[test]
    fn test_config_path_respects_home() {
        let original_home = std::env::var("HOME").ok();
        std::env::set_var("HOME", "/home/testuser");

        let path = config_path();

        if let Some(original) = original_home {
            std::env::set_var("HOME", original);
        }

        assert!(path.is_some());
        assert!(path.unwrap().to_string_lossy().contains("testuser"));
    }
}
