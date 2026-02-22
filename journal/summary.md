# rsync_tui - Implementation Summary

## Project Overview
A terminal user interface (TUI) for rsync, inspired by lazygit's clean design and the Charm ecosystem's visual aesthetic.

## Technology Stack
- Language: Rust
- TUI Library: Ratatui 0.27
- Terminal Backend: Crossterm (via ratatui re-export)
- Config Format: JSON (planned)
- Transport: rsync with SSH support

## Current Status: MVP Complete + Visual Modernization

### Implemented Features
- Panel-based TUI layout (Source, Destination, Options, Logs, Progress)
- Keyboard navigation between panels (Tab, Shift+Tab, j/k, 1-5)
- Vim-style Normal/Insert modes
- 10 toggleable rsync options (letter keys in Normal mode)
- Live command preview with text wrapping
- Rsync execution with progress capture
- Dry-run support (Ctrl+n)
- Path autocomplete (Tab in Insert mode)
- Charm-inspired visual theme (rounded borders, pill badges, RGB colors)

### Rsync Options Supported
| Key | Flag | Description |
|-----|------|-------------|
| a | `-a` | Archive mode |
| v | `-v` | Verbose |
| z | `-z` | Compress |
| n | `-n` | Dry-run |
| p | `--progress` | Progress per file |
| d | `--delete` | Delete extraneous |
| h | `-h` | Human-readable |
| e | `-e ssh` | Use SSH |
| r | `--remove-source-files` | Delete source |
| f | `--info=progress2` | Global progress |

## Development Timeline
- 2025-02-01: Project planning and initial setup
- 2026-02-02: MVP implementation (core structure, TUI, rsync integration)
- 2026-02-22: Visual modernization (Charm-inspired theme, ratatui 0.27 upgrade)

## Architecture

```
src/
├── main.rs           # Entry point, event loop, keyboard handling
├── app.rs            # Application state (App, Panel, Mode)
├── event.rs          # Keyboard event polling
├── path.rs           # Path autocomplete with tilde expansion
├── ui/
│   ├── mod.rs        # Module declarations
│   ├── layout.rs     # Panel rendering (7 render functions)
│   └── theme.rs      # Centralized color palette and style helpers
└── rsync/
    ├── mod.rs        # Module declarations
    ├── command.rs    # Command builder and formatter
    └── options.rs    # RsyncOptions struct and toggle logic
```

## Design Principles
- Functional over object-oriented
- Simple, readable code for non-Rust developers
- Modular: each file handles one responsibility
- MVP-first: 80% of features with 20% of code
- Centralized theming via theme.rs

## Pending Features
- JSON configuration persistence
- Exclude pattern UI
- Upgrade Rust toolchain (nix) for ratatui 0.30+
