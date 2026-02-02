# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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
