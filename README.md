# Rsync TUI

A terminal user interface for rsync, inspired by lazygit's clean design.

## Features

- Interactive panel-based interface
- Toggle common rsync options with number keys
- Live command preview
- Support for local and remote (SSH) transfers
- Log output display

## Requirements

- Rust 1.70+ (for building)
- rsync installed on your system

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rsync_tui.git
cd rsync_tui

# Build the project
cargo build --release

# Run the binary
./target/release/rsync_tui
```

## Usage

### Starting the Application

```bash
cargo run
# or after building:
./target/release/rsync_tui
```

### Interface Layout

```
┌─────────────────────────────────────────────────────────────┐
│  rsync TUI                                                  │
├──────────────────────┬──────────────────────────────────────┤
│ Source               │ Destination                          │
│ /path/to/source      │ user@host:/path/to/dest              │
├──────────────────────┴──────────────────────────────────────┤
│ Options (press number to toggle)                            │
│ [x]1 Archive  [x]2 Verbose  [ ]3 Compress  [ ]4 Dry-run    │
├─────────────────────────────────────────────────────────────┤
│ Preview / Logs                                              │
│ > rsync -avh --progress /path/to/source user@host:/dest     │
├─────────────────────────────────────────────────────────────┤
│ [Tab] Switch panel  [Ctrl+s] Sync  [Ctrl+n] Dry-run  [q] Quit│
└─────────────────────────────────────────────────────────────┘
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Tab` | Switch to next panel |
| `Shift+Tab` | Switch to previous panel |
| `1-8` | Toggle rsync options |
| `Ctrl+s` | Execute rsync sync |
| `Ctrl+n` | Execute dry-run (preview only) |
| `Backspace` | Delete character in path input |
| `q` | Quit application |
| `Ctrl+c` | Quit application |

### Rsync Options

| Key | Option | Flag | Description |
|-----|--------|------|-------------|
| `1` | Archive | `-a` | Recursive + preserve permissions, timestamps, symlinks |
| `2` | Verbose | `-v` | Show files being transferred |
| `3` | Compress | `-z` | Compress data during transfer |
| `4` | Dry-run | `-n` | Preview without making changes |
| `5` | Progress | `--progress` | Show transfer progress |
| `6` | Delete | `--delete` | Delete extraneous files on destination |
| `7` | Human | `-h` | Human-readable file sizes |
| `8` | SSH | `-e ssh` | Use SSH for remote transfers |

### Examples

**Local sync:**
- Source: `/home/user/documents`
- Destination: `/backup/documents`

**Remote sync (SSH):**
- Source: `/home/user/documents`
- Destination: `user@server:/backup/documents`
- Enable option `8` (SSH)

## Project Structure

```
src/
├── main.rs           # Entry point, event loop
├── app.rs            # Application state
├── event.rs          # Keyboard event handling
├── ui/
│   ├── mod.rs        # UI module
│   └── layout.rs     # Panel rendering
└── rsync/
    ├── mod.rs        # Rsync module
    ├── command.rs    # Command builder
    └── options.rs    # Option definitions
```

## License

MIT
