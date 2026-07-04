use std::io::{BufRead, BufReader, Read};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/// Event emitted by a running rsync process
pub enum RsyncEvent {
    /// A line of output (stderr lines are prefixed with "[ERR] ")
    Line(String),
    /// Parsed progress: percentage (0-100) and transfer info
    Progress(f64, String),
}

/// Handle to a running rsync process
pub struct RsyncRunner {
    pub child: Child,
    pub events: Receiver<RsyncEvent>,
}

/// Spawn rsync in the background, streaming output events over a channel.
/// `global` marks progress percentages as whole-transfer (--info=progress2)
/// rather than per-file (--progress).
pub fn spawn(args: &[String], global: bool) -> std::io::Result<RsyncRunner> {
    let mut child = Command::new(&args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let (tx, rx) = channel();

    if let Some(stdout) = child.stdout.take() {
        let tx = tx.clone();
        thread::spawn(move || stream_output(stdout, &tx, false, global));
    }
    if let Some(stderr) = child.stderr.take() {
        let tx = tx.clone();
        thread::spawn(move || stream_output(stderr, &tx, true, global));
    }

    Ok(RsyncRunner { child, events: rx })
}

/// Read process output, splitting on both \n and \r so that rsync's
/// carriage-return progress updates arrive as they happen.
fn stream_output<R: Read>(source: R, tx: &Sender<RsyncEvent>, is_err: bool, global: bool) {
    let mut reader = BufReader::new(source);
    let mut acc: Vec<u8> = Vec::new();
    loop {
        let chunk = match reader.fill_buf() {
            Ok(chunk) if chunk.is_empty() => break,
            Ok(chunk) => chunk.to_vec(),
            Err(_) => break,
        };
        reader.consume(chunk.len());
        for byte in chunk {
            if byte == b'\r' || byte == b'\n' {
                emit_line(&mut acc, tx, is_err, global);
            } else {
                acc.push(byte);
            }
        }
    }
    emit_line(&mut acc, tx, is_err, global);
}

/// Send the accumulated bytes as events and clear the buffer
fn emit_line(acc: &mut Vec<u8>, tx: &Sender<RsyncEvent>, is_err: bool, global: bool) {
    if acc.is_empty() {
        return;
    }
    let line = String::from_utf8_lossy(acc).to_string();
    acc.clear();

    if is_err {
        let _ = tx.send(RsyncEvent::Line(format!("[ERR] {}", line)));
        return;
    }
    if let Some((percent, info)) = parse_progress(&line) {
        let info = if global { info } else { format!("{} (file)", info) };
        let _ = tx.send(RsyncEvent::Progress(percent, info));
    }
    let _ = tx.send(RsyncEvent::Line(line));
}

/// Parse a progress line like "  1,234,567  45%  12.34MB/s  0:01:23"
pub fn parse_progress(line: &str) -> Option<(f64, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if part.ends_with('%') {
            if let Ok(percent) = part.trim_end_matches('%').parse::<f64>() {
                let info: Vec<&str> = parts[i + 1..].iter().take(2).copied().collect();
                return Some((percent.min(100.0), info.join(" ")));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    fn collect_events(data: &[u8], is_err: bool, global: bool) -> Vec<RsyncEvent> {
        let (tx, rx) = channel();
        stream_output(data, &tx, is_err, global);
        drop(tx);
        rx.try_iter().collect()
    }

    #[test]
    fn test_stream_splits_on_carriage_return() {
        let data = b"created dir\n 1,000 45% 1.2MB/s 0:00:10\r 2,000 90% 1.3MB/s 0:00:02\r";
        let events = collect_events(data, false, true);

        let progress: Vec<f64> = events
            .iter()
            .filter_map(|e| match e {
                RsyncEvent::Progress(p, _) => Some(*p),
                _ => None,
            })
            .collect();
        assert_eq!(progress, vec![45.0, 90.0]);
    }

    #[test]
    fn test_stream_marks_stderr() {
        let events = collect_events(b"boom\n", true, true);

        assert!(matches!(&events[0], RsyncEvent::Line(l) if l == "[ERR] boom"));
    }

    #[test]
    fn test_per_file_progress_labelled() {
        let events = collect_events(b" 1,000 45% 1.2MB/s 0:00:10\n", false, false);

        assert!(events
            .iter()
            .any(|e| matches!(e, RsyncEvent::Progress(_, info) if info.ends_with("(file)"))));
    }

    #[test]
    fn test_parse_progress_extracts_percent_and_info() {
        let (percent, info) = parse_progress("  1,234  45%  12.3MB/s  0:01:23").unwrap();

        assert_eq!(percent, 45.0);
        assert_eq!(info, "12.3MB/s 0:01:23");
    }

    #[test]
    fn test_parse_progress_ignores_plain_lines() {
        assert!(parse_progress("sending incremental file list").is_none());
    }

    #[test]
    fn test_parse_progress_clamps_over_100() {
        let (percent, _) = parse_progress(" 200% ").unwrap();

        assert_eq!(percent, 100.0);
    }
}
