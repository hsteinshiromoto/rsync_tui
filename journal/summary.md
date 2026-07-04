# rsync_tui - Implementation Summary

## Project Overview
A terminal user interface (TUI) for rsync, inspired by lazygit's clean design and the Charm ecosystem's visual aesthetic.

## Technology Stack
- Language: Rust
- TUI Library: Ratatui 0.27
- Terminal Backend: Crossterm (via ratatui re-export)
- Config Format: JSON (planned)
- Transport: rsync with SSH support

## Current Status: MVP Complete + Visual Modernization + Improvement Plan

A full application review (2026-07-04) produced `docs/improvement_plan.md`: a three-phase roadmap fixing critical execution-core defects (synchronous rsync freezing the UI, a dry-run that deletes empty source dirs, pipe-deadlock risk), then UX/architecture work on ratatui 0.27, then the toolchain upgrade to ratatui 0.30.

### Implemented Features
- Panel-based TUI layout (Source, Destination, Options, Logs, Progress)
- Keyboard navigation between panels (Tab, Shift+Tab, j/k, 1-5)
- Vim-style Normal/Insert modes
- 10 toggleable rsync options (letter keys in Normal mode)
- Live command preview with text wrapping
- Rsync execution with progress capture
- Dry-run support (Ctrl+n)
- Path autocomplete (Tab in Insert mode)
- Tokyo Night visual theme (rounded borders, pill badges, RGB colors)

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
- 2026-02-22: Tokyo Night color scheme, consolidated duplicate theme constants
- 2026-07-04: Full application review and critique; improvement plan created at docs/improvement_plan.md (no code changes)

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
Now tracked in `docs/improvement_plan.md` as a prioritised three-phase roadmap:
- Phase 0: safety fixes (dry-run delete guard, panic hook, Makefile fix, dependency pruning)
- Phase 1: threaded rsync runner (live progress, cancel), confirmation modal, cursor editing, JSON configuration persistence, exclude pattern UI, UI polish workstream (compact state-driven layout, running-state feedback, danger styling, path-input affordances, help overlay, 256-colour fallback)
- Phase 2: upgrade Rust toolchain (nix) for ratatui 0.30+ (nix env currently inconsistent: rustc 1.81 vs clippy 1.95)
