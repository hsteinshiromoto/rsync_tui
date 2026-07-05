# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added
- Live progress during transfers: rsync now runs on a background thread and the UI keeps redrawing, updating the gauge as carriage-return progress updates arrive
- Cancel a running transfer with `Esc` (Normal mode) or `Ctrl+c`; a RUNNING indicator shows in the title bar
- Starting a second transfer while one is running is refused
- Confirmation modal before any run with destructive options (`--delete`, `--remove-source-files`) and before cancelling a running transfer; `y`/`Enter` confirms, `n`/`Esc` dismisses
- Destructive options render as red pills when enabled
- Cursor editing in Insert mode: Left/Right/Home/End keys to navigate, mid-string insert/delete (no longer append-only), Delete key, Backspace
- Settings persist to `~/.config/rsync_tui/config.json` and are loaded on startup
- Scrollable logs and progress output; Page Up/Down to scroll in Normal mode
- Terminal size check: graceful message if terminal too small (&lt; 60×20)

### Fixed
- UI no longer freezes for the duration of a transfer
- stderr is read on its own thread, eliminating a pipe-buffer deadlock on error-heavy runs
- The per-file progress toggle is no longer silently forced on; user toggles are honoured
- Per-file progress percentages are labelled "(file)" so they are not mistaken for whole-transfer progress
- Command preview no longer panics when wrapping multi-byte (non-ASCII) paths
- Dry-run no longer deletes empty source directories when Delete source (`r`) is enabled — the post-run cleanup is skipped for previews and hidden from the command preview
- Panics now restore the terminal (raw mode / alternate screen) before printing, so the shell stays usable
- `make test` works again (`.PHONY` was missing its colon)
- Runs are refused with a clear message when source or destination is empty
- rsync failures report human-readable exit codes (e.g. "23 — partial transfer due to error") instead of `Some(23)`

### Changed
- Options are now defined in one table (`OPTIONS`) driving key toggles, pills, the help bar, and destructive-run detection
- Progress option fields renamed to match their flags: `per_file_progress` (`--progress`), `global_progress` (`--info=progress2`)
- Log buffer bounded at 1000 lines (previously unbounded, stored twice)
- README documents all 10 options (added `r` Delete source, `f` Global progress) and the Rust 1.81 toolchain

### Removed
- Unused `tokio` dependency and dead code (`event::is_quit`)

## [0.1.0] - 2026-02-02

### Added
- **Panel-based TUI** with 5 panels:
  - [1] Source - Enter source path
  - [2] Destination - Enter destination path
  - [3] Options - Toggle rsync flags
  - [4] Preview/Logs - Command preview and output logs
  - [5] Progress - Real-time progress bar and transfer output
- **Vim-style Modes**:
  - `[NORMAL]` mode for panel navigation and option toggles
  - `[INSERT]` mode for text editing in Source/Destination panels
  - Press `i` to enter Insert mode, `Esc` to return to Normal mode
- **Keyboard Navigation**:
  - `1-5` keys to jump directly to panels
  - `j`/`k` or `Tab`/`Shift+Tab` for sequential navigation
  - `Enter` in Logs panel to execute rsync
  - `Enter` in Insert mode to move to next panel
- **Path Autocomplete**: Press `Tab` in Insert mode
  - Supports tilde (`~`) expansion for home directory
  - Completes partial paths with common prefix matching
- **8 Rsync Options** (toggle with letter keys):
  - `a` Archive, `v` Verbose, `z` Compress, `n` Dry-run
  - `p` Progress, `d` Delete, `h` Human-readable, `e` SSH
- **Progress Tracking**:
  - Progress bar with percentage and transfer speed
  - Live rsync terminal output display
- **Rsync Execution**:
  - `Ctrl+s` to sync, `Ctrl+n` for dry-run
  - SSH support for remote transfers
