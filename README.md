# Rsync TUI

A terminal user interface for rsync, inspired by lazygit's clean design.

## Features

- Interactive panel-based interface with vim-style modes (Normal/Insert)
- Toggle rsync options with letter keys (a/v/z/n/p/d/h/e)
- Live command preview
- Real-time progress bar with transfer speed display
- Path autocomplete with Tab key
- Support for local and remote (SSH) transfers
- Vim-style navigation (j/k keys)

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
│  rsync TUI [NORMAL]                                         │
├─────────────────────────────────────────────────────────────┤
│ [1] Source                                                  │
│ /path/to/source                                             │
├─────────────────────────────────────────────────────────────┤
│ [2] Destination                                             │
│ user@host:/path/to/dest                                     │
├─────────────────────────────────────────────────────────────┤
│ [3] Options                                                 │
│ [x]a Archive  [x]v Verbose  [ ]z Compress  [ ]n Dry-run    │
├─────────────────────────────────────────────────────────────┤
│ [4] Preview / Logs                                          │
│ > rsync -avh --progress /path/to/source user@host:/dest     │
├─────────────────────────────────────────────────────────────┤
│ [5] Progress                                                │
│ ████████████████████░░░░░░░░░░░░░░░░░░░░  45% - 12.3MB/s   │
│ sending file1.txt                                           │
├─────────────────────────────────────────────────────────────┤
│ [1-5/j/k] Panels  [i] Insert  [a/v/z/n/p/d/h/e] Options    │
└─────────────────────────────────────────────────────────────┘
```

### Vim Modes

The application uses vim-style modes:

- **Normal Mode** `[NORMAL]`: Navigate panels and toggle options
- **Insert Mode** `[INSERT]`: Edit text in Source/Destination panels

### Keyboard Shortcuts

#### Normal Mode

| Key | Action |
|-----|--------|
| `1-5` | Jump to panel (Source, Destination, Options, Logs, Progress) |
| `j` / `Tab` | Move to next panel |
| `k` / `Shift+Tab` | Move to previous panel |
| `i` | Enter Insert mode (in Source/Destination panels) |
| `a/v/z/n/p/d/h/e` | Toggle rsync options |
| `Enter` | Execute rsync (when in Logs panel) |
| `Ctrl+s` | Execute rsync sync |
| `Ctrl+n` | Execute dry-run (preview only) |
| `q` / `Ctrl+c` | Quit application |

#### Insert Mode

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal mode |
| `Enter` | Move to next panel (stays in Insert if applicable) |
| `Tab` | Path autocomplete |
| `Backspace` | Delete character |
| `Ctrl+s` | Execute rsync sync |
| `Ctrl+n` | Execute dry-run |

### Rsync Options

| Key | Option | Flag | Description |
|-----|--------|------|-------------|
| `a` | Archive | `-a` | Recursive + preserve permissions, timestamps, symlinks |
| `v` | Verbose | `-v` | Show files being transferred |
| `z` | Compress | `-z` | Compress data during transfer |
| `n` | Dry-run | `-n` | Preview without making changes |
| `p` | Progress | `--progress` | Show transfer progress |
| `d` | Delete | `--delete` | Delete extraneous files on destination |
| `h` | Human | `-h` | Human-readable file sizes |
| `e` | SSH | `-e ssh` | Use SSH for remote transfers |

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
├── app.rs            # Application state (panels, modes)
├── event.rs          # Keyboard event handling
├── path.rs           # Path autocomplete utilities
├── ui/
│   ├── mod.rs        # UI module
│   └── layout.rs     # Panel rendering (including progress bar)
└── rsync/
    ├── mod.rs        # Rsync module
    ├── command.rs    # Command builder
    └── options.rs    # Option definitions
```

## License

MIT
