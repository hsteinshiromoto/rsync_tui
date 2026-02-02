# rsync_tui - Implementation Summary

## Project Overview
A terminal user interface (TUI) for rsync, inspired by lazygit's clean design.

## Technology Stack
- Language: Rust
- TUI Library: Ratatui 0.26
- Terminal Backend: Crossterm 0.27
- Config Format: JSON (planned)
- Transport: rsync with SSH support

## Current Status: MVP Complete

### Implemented Features
- Panel-based TUI layout (Source, Destination, Options, Logs)
- Keyboard navigation between panels
- 8 toggleable rsync options (1-8 keys)
- Live command preview
- Rsync execution with output capture
- Dry-run support (Ctrl+n)

### Rsync Options Supported
| Key | Flag | Description |
|-----|------|-------------|
| 1 | `-a` | Archive mode |
| 2 | `-v` | Verbose |
| 3 | `-z` | Compress |
| 4 | `-n` | Dry-run |
| 5 | `--progress` | Progress |
| 6 | `--delete` | Delete extraneous |
| 7 | `-h` | Human-readable |
| 8 | `-e ssh` | Use SSH |

## Development Timeline
- 2025-02-01: Project planning and initial setup
- 2026-02-02: MVP implementation (core structure, TUI, rsync integration)

## Architecture

```
src/
├── main.rs           # Entry point, event loop
├── app.rs            # Application state (App, Panel)
├── event.rs          # Keyboard event handling
├── ui/
│   └── layout.rs     # Panel rendering
└── rsync/
    ├── command.rs    # Command builder
    └── options.rs    # Option definitions
```

## Design Principles
- Functional over object-oriented
- Simple, readable code for non-Rust developers
- Modular: each file handles one responsibility
- MVP-first: 80% of features with 20% of code

## Pending Features
- JSON configuration persistence
- Exclude pattern UI
- Unit tests
- Path autocomplete
