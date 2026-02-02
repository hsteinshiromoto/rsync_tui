use std::fs;
use std::path::Path;

/// Complete a partial path using the filesystem.
/// Returns the completed path if matches found, None otherwise.
pub fn complete_path(partial: &str) -> Option<String> {
    if partial.is_empty() {
        return None;
    }

    let path = Path::new(partial);

    // If the path exists and is a directory, list its contents
    if path.is_dir() && partial.ends_with('/') {
        return None; // Already complete directory, nothing to complete
    }

    // Get parent directory and the prefix to match
    let (parent_dir, prefix) = if path.is_dir() {
        // Path is a directory without trailing slash, add it
        return Some(format!("{}/", partial));
    } else {
        // Get parent and filename prefix
        let parent = path.parent().unwrap_or(Path::new("."));
        // Fix: handle empty parent (for relative paths like "sr")
        let parent = if parent.as_os_str().is_empty() {
            Path::new(".")
        } else {
            parent
        };
        let prefix = path.file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        (parent, prefix)
    };

    // Read directory entries
    let entries = match fs::read_dir(parent_dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    // Find matching entries
    let matches: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) {
                Some(e.path().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    match matches.len() {
        0 => None,
        1 => {
            // Single match - complete fully
            let completed = &matches[0];
            if Path::new(completed).is_dir() {
                Some(format!("{}/", completed))
            } else {
                Some(completed.clone())
            }
        }
        _ => {
            // Multiple matches - find common prefix
            let common = find_common_prefix(&matches);
            if common.len() > partial.len() {
                Some(common)
            } else {
                None // No additional completion possible
            }
        }
    }
}

/// Find the common prefix among multiple strings
fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    if strings.len() == 1 {
        return strings[0].clone();
    }

    let first = &strings[0];
    let mut prefix_len = first.len();

    for s in &strings[1..] {
        prefix_len = first
            .chars()
            .zip(s.chars())
            .take(prefix_len)
            .take_while(|(a, b)| a == b)
            .count();
    }

    first.chars().take(prefix_len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_complete_home_directory() {
        // Test with a path that should exist on most systems
        let home = env::var("HOME").unwrap_or_default();
        if !home.is_empty() {
            let partial = &home[..home.len().saturating_sub(2)];
            let result = complete_path(partial);
            assert!(result.is_some());
        }
    }

    #[test]
    fn test_complete_nonexistent_path() {
        let result = complete_path("/nonexistent_path_12345/");
        assert!(result.is_none());
    }

    #[test]
    fn test_complete_empty_string() {
        let result = complete_path("");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_common_prefix() {
        let strings = vec![
            "hello_world".to_string(),
            "hello_there".to_string(),
            "hello_rust".to_string(),
        ];
        assert_eq!(find_common_prefix(&strings), "hello_");
    }

    #[test]
    fn test_find_common_prefix_single() {
        let strings = vec!["hello".to_string()];
        assert_eq!(find_common_prefix(&strings), "hello");
    }

    #[test]
    fn test_find_common_prefix_empty() {
        let strings: Vec<String> = vec![];
        assert_eq!(find_common_prefix(&strings), "");
    }

    #[test]
    fn test_complete_relative_path() {
        // Test relative path completion (from current directory)
        // This assumes we're in a directory with a "src" folder
        let result = complete_path("sr");
        // Should either complete to something or return None (depending on cwd)
        // Just verify it doesn't panic
        let _ = result;
    }
}
