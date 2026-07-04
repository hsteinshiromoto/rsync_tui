# rsync_tui Improvement Plan

**Status:** Approved draft | **Date:** 2026-07-04 | **Baseline:** branch `dev` (clean), rsync_tui v0.1.0
**Constraints:** Stay on ratatui. Phase 1 targets ratatui 0.27 / Rust 1.81; Phase 2 upgrades the toolchain. Functional, modular, minimal (80/20) style; standard libraries preferred.
**Review provenance:** full-source review (all 10 files under `src/`, `Cargo.toml`, `Makefile`, `README.md`), verified by an independent plan evaluation. `cargo check` clean, `cargo test` 35/35 pass on rustc 1.81.0.

## 1. Executive summary

rsync_tui is a working ratatui prototype with a clean, well-themed visual layer but a broken execution core: rsync runs **synchronously inside the event loop**, so the UI freezes for the whole transfer, the advertised "real-time progress bar" never updates, and a run cannot be cancelled. Worse, a **dry-run with delete-source enabled actually deletes empty source directories** — a data-integrity bug in what users treat as a safe preview. The codebase also carries unused heavyweight dependencies (tokio "full"), option/key mappings duplicated across three files, unbounded log buffers, and a Makefile that fails to parse. This plan fixes safety issues immediately (Phase 0), rebuilds the runner and UX on the current toolchain (Phase 1), then upgrades the nix toolchain and migrates to ratatui 0.30 (Phase 2).

## 2. Critique summary

| ID | Sev | Location | Finding |
|----|-----|----------|---------|
| C1 | Critical | `src/main.rs:177-276` | `run_rsync` runs synchronously in the event loop; UI frozen for the whole transfer, no live progress, no cancel; tokio declared but unused |
| C2 | Critical | `src/main.rs:234-260` | `find <source> -type d -empty -delete` not guarded by `dry_run` — a dry-run with DelSrc on deletes empty source dirs |
| C3 | Critical | `src/main.rs:207-231` | stdout drained to EOF before stderr → pipe-buffer deadlock risk (~64 KB) on stderr-heavy runs |
| C4 | Critical | `src/main.rs:209,188` | rsync progress lines are `\r`-terminated; `BufReader::lines()` misses live updates; `opts.progress` force-set to true, overriding the user's toggle |
| C5 | Critical | `src/rsync/options.rs:13`, `src/main.rs:280-295` | Field `progress_per_file` actually holds `--info=progress2` (global progress) — name inverted; `parse_progress` is mode-blind (treats per-file % as global). UI pill labels themselves are semantically correct |
| H1 | High | `src/main.rs:16-34` | No panic hook — a panic leaves the terminal in raw mode / alternate screen |
| H2 | High | `src/main.rs:154-171` | Insert mode is append/pop only; no cursor rendered (`frame.set_cursor` never called), no mid-string editing |
| H3 | High | keymap + `src/main.rs` | `--delete` / `--remove-source-files` run via a single Ctrl+s with no confirmation |
| H4 | High | `src/app.rs:31-33`, `src/main.rs:216-217` | Every output line stored twice (logs + progress_output), unbounded; renders use only the last 20/10 lines |
| H5 | High | `src/ui/layout.rs:15-26` | Fixed `Length` constraints (~31 rows min); small terminals silently truncate; no scrolling anywhere |
| H6 | High | `src/rsync/command.rs:39-42` | Exclude patterns supported by the command builder but unreachable from the UI |
| H7 | High | `src/main.rs:262` | Exit status displayed via `{:?}` ("Some(23)"); rsync exit codes have well-known meanings |
| H8 | High | `src/main.rs:40-56`, `src/app.rs:28` | No guard against empty source/destination (spawns `rsync "" ""`); `app.running` never checked, so run keys can spawn concurrent transfers once the loop is responsive |
| M1 | Med | `src/main.rs` | Event loop, key handling, process execution, and progress parsing all in one file |
| M2 | Med | `src/rsync/options.rs:37-51`, `src/main.rs:108-117`, `src/ui/layout.rs:114-125` | Option key/label/flag mappings duplicated in 3+ places via magic indices 0-9 |
| M3 | Med | `src/ui/layout.rs:190-240` | `wrap_command` slices by byte index (`remaining[..wrap_at]`, `split_at`) — panics on multi-byte UTF-8 paths |
| M4 | Med | `Cargo.toml`, `src/event.rs:16-29` | Unused tokio ("full" drags ~100 deps into every build); serde unused until F1; dead `is_quit` |
| M5 | Med | `src/ui/layout.rs:59-107` | `render_source` / `render_destination` are near-duplicates |
| M6 | Med | `src/app.rs:37` | `App::new()` should implement `Default` |
| M7 | Med | keymap | Option toggles fire from any panel in Normal mode; `h` conflicts with vim-motion muscle memory; j/k wrap order surprising |
| M8 | Med | inline test modules, `src/path.rs` | 35 tests but zero coverage of `parse_progress`, `wrap_command`, or the runner; `$HOME`/cwd-dependent tests are flaky; no C2/C4 regression tests |
| M9 | Med | `Makefile:1` | `.PHONY test` missing colon — `make test` fails "missing separator" (verified) |
| M10 | Med | `README.md`, `CHANGELOG.md` | README lists 8 options (actual 10; r/f missing) and cites Rust 1.70+ — align with the dev toolchain (1.81); "real-time progress" claim untrue until C1 lands; CHANGELOG stale at v0.1.0 |
| M11 | Med | `src/rsync/command.rs` | `-e ssh` is redundant (rsync's default for `host:` paths); drop or repurpose for custom ssh args |
| T1 | Toolchain | nix config | rustc 1.81.0 with clippy 0.1.95 (built from Rust 1.95) → E0514; clippy currently unusable |
| T2 | Toolchain | `Cargo.toml`, `src/` | ratatui 0.27 → 0.30 migration once toolchain ≥ 1.88 |
| F1 | Feature | — | JSON config persistence at `~/.config/rsync_tui/config.json` (serde already declared) |
| F2 | Feature | — | Exclude-pattern editor UI (unblocks H6) |
| U1 | UX | `src/ui/layout.rs:15-26` | Layout needs ~31 rows: 3-row title bar holds one line; Options panel wastes a spacer row; Progress panel reserves 6+ rows even when idle |
| U2 | UX | `src/app.rs:28`, `src/ui/layout.rs:242-300` | Running state never surfaced (no spinner, elapsed time, or completion badge); gauge shows a meaningless "0%" when idle; no dry-run summary |
| U3 | UX | `src/ui/layout.rs:165` | Logs render newest-first at the top — reads upside-down next to the command preview; expected newest-at-bottom with stick-to-bottom scrolling |
| U4 | UX | `src/ui/layout.rs:345-363`, `render_logs` | Destructive toggles (d Delete, r DelSrc) render in the same blue as harmless ones when enabled, and destructive flags aren't highlighted in the command preview; no in-panel option navigation or flag descriptions |
| U5 | UX | `src/main.rs:137-151`, `src/path.rs` | Tab completion is invisible until pressed (no ghost-text preview); no live source-path validation; no hint for rsync's trailing-slash semantics (copy dir vs contents) |
| U6 | UX | `src/ui/layout.rs:302-343` | One-line help bar clips on narrow terminals; no full-help overlay |
| U7 | UX | `src/ui/theme.rs` | Pure `Color::Rgb` palette degrades on non-truecolor terminals (e.g. default macOS Terminal.app) and assumes a dark background |

## 3. Phased roadmap

### Phase 0 — Safety & hygiene (immediate; all small, independent, shippable same-day)

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| 0.1 | Guard the post-run `find … -delete` cleanup behind `!opts.dry_run`; add regression test | C2 | S | P0 |
| 0.2 | `std::panic::set_hook` that restores the terminal (raw mode off, leave alt screen) before printing the panic | H1 | S | P0 |
| 0.3 | Fix `Makefile` (`.PHONY: test`) | M9 | S | P0 |
| 0.4 | Prune dependencies: remove tokio and dead `is_quit`; keep serde/serde_json (needed for E.1); keep or drop anyhow per remaining usage | M4 | S | P0 |
| 0.5 | Refuse to run when source or destination is empty (log a clear message) | H8 | S | P0 |
| 0.6 | Map rsync exit codes to human-readable messages (0, 12, 23, 24, … with fallback to raw code) | H7 | S | P1 |
| 0.7 | Docs pass: README option table (10 options incl. r/f), align toolchain statement to 1.81, soften "real-time progress" claim until A-workstream lands; CHANGELOG entry | M10 | S | P1 |

### Phase 1 — Core architecture & UX (ratatui 0.27 / Rust 1.81; all items verified feasible on 0.27)

**WS-A: Threaded rsync runner** (the centrepiece)

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| A.1 | Extract `src/rsync/runner.rs`: spawn rsync on `std::thread` with an `std::sync::mpsc` channel of `RsyncEvent { Progress(f64, String), Line(String), Done(ExitInfo) }`; event loop polls channel + input each tick; cancel via `Child::kill` (Esc/Ctrl+c during run); ignore run keys while `app.running` and surface the running state in the UI | C1, M1, H8 | L | P0 |
| A.2 | Read stderr on its own thread (second sender into the same channel) | C3 | S (with A.1) | P0 |
| A.3 | `\r`-aware output reading (`read_until(b'\r')` with `\n` handling) so per-file progress arrives live; stop force-setting `opts.progress` | C4 | M | P0 |
| A.4 | Fix progress-flag semantics: rename fields to match flags (`per_file_progress` = `--progress`, `global_progress` = `--info=progress2`); make `parse_progress` mode-aware | C5 | M | P0 |

**WS-B: Safety & input UX**

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| B.1 | Confirmation modal before any run with destructive flags (`--delete`, `--remove-source-files`) — `Clear` widget + centred `Rect` (available in 0.27). Pre-run modal is independent of WS-A; the cancel-mid-run confirm depends on A.1 | H3 | M | P0 |
| B.2 | Real cursor editing in Insert mode: `frame.set_cursor` (0.27 API), Left/Right/Home/End, char-index-safe mid-string insert/delete | H2 | M | P1 |
| B.3 | Bounded output buffer: one `VecDeque` capped at ~1000 lines; drop the duplicate `progress_output` copy | H4 | S | P1 |
| B.4 | Scrollable logs/progress panels + graceful small-terminal handling ("terminal too small" notice) | H5 | M | P1 |
| B.5 | Keymap tidy: scope option toggles to the Options panel (or document as global); revisit `h` vs vim motions; fix j/k wrap order | M7 | S | P2 |

**WS-C: Data-driven refactors**

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| C.1 | Single `OPTIONS: &[OptionDef { key, label, flag, destructive, accessor }]` table driving toggling, pills, help bar, and destructive-flag detection for B.1 — removes magic indices across three files | M2 | M | P1 |
| C.2 | Replace byte-sliced `wrap_command` with char-aware wrapping (or `Paragraph::wrap`) | M3 | S | P1 |
| C.3 | Factor `render_path_input(frame, area, label, badge, value, active)` from the source/destination duplicates | M5 | S | P2 |
| C.4 | `impl Default for App`; misc tidy-ups (full lint sweep deferred to Phase 2 when clippy works) | M6 | S | P2 |
| C.5 | Decide `-e ssh`: drop (minimal-code standard) or keep as the hook for custom ssh args (port/key) later | M11 | S | P2 |

**WS-D: Tests**

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| D.1 | Unit tests: `parse_progress` (both modes), command wrapping (multi-byte input), runner event stream (fake command), and the C2 dry-run guard | M8 | M | P1 |
| D.2 | Make `path.rs` tests hermetic (temp dirs instead of `$HOME`/cwd) | M8 | S | P1 |

**WS-E: Features**

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| E.1 | JSON config persistence: save/restore source, destination, options at `~/.config/rsync_tui/config.json` (load on start, save on quit/run) | F1 | M | P1 |
| E.2 | Exclude-pattern editor UI (add/remove list in the Options panel), wiring the already-working `command.rs` support | F2, H6 | M | P1 |

**WS-U: UI polish**

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| U.1 | Compact, state-driven layout: single-line title (mode badge right-aligned), drop the Options spacer row, collapse the Progress panel when idle and expand it during a run — target usable at 80×24 | U1, H5 | M | P1 |
| U.2 | Running-state feedback: spinner + elapsed time while a transfer runs, coloured Progress border during the run, completion badge ("✓ completed" / "✗ 23 — partial transfer" via the 0.6 exit-code map); idle gauge placeholder ("idle — Ctrl+s to sync"); dry-run summary line | U2 | M | P1 |
| U.3 | Logs newest-at-bottom with stick-to-bottom scrolling (implement together with B.4) | U3 | S | P1 |
| U.4 | Danger styling: destructive toggles (d/r) get warning colour (red/orange pill) when enabled and destructive flags render red in the command preview; focused-option navigation within the Options panel (h/l + Space) with a one-line description of the highlighted flag | U4 | M | P1 |
| U.5 | Path-input affordances: ghost-text completion preview (dim inline suggestion), live source-path validation (red border when missing locally), trailing-slash semantics hint ("copies directory itself" vs "copies contents") | U5 | M | P1 |
| U.6 | `?` full-help overlay modal listing all keybindings per mode (Clear widget + centred Rect, shares B.1's modal plumbing) | U6 | S | P2 |
| U.7 | Terminal compatibility: 256-colour fallback (`Color::Indexed` approximations) when truecolor is unavailable; audit legibility on light backgrounds | U7 | M | P2 |

### Phase 2 — Toolchain & ratatui 0.30

| # | Item | IDs | Effort | Priority |
|---|------|-----|--------|----------|
| 2.1 | Fix the nix config to one coherent toolchain (rustc ≥ 1.88 with matching clippy/rustfmt) — resolves the current rustc 1.81 / clippy 1.95 mismatch | T1 | M | P0 |
| 2.2 | Migrate ratatui 0.27 → 0.30: `frame.size()` → `frame.area()`, `set_cursor` → `set_cursor_position`, crossterm re-export paths, new Stylize idioms | T2 | M | P0 |
| 2.3 | Re-enable clippy in Makefile/CI; fix new lints | T1, T2 | S | P1 |

## 4. Dependency notes

- **A.1 is the keystone**: cancel support, the stderr thread (A.2), live parsing (A.3), the run-key guard, and the *mid-run* cancel-confirm all presuppose the threaded runner. Land A.1–A.4 as one workstream — they live in the same extracted module.
- **B.1's pre-run confirmation modal is NOT blocked by A.1** — it works in the current synchronous loop too; only the cancel-mid-run confirm depends on the runner.
- **A.4 (naming) lands with A.3 (parsing)** — same code paths; separating them means touching `parse_progress` twice.
- **M1's extraction is the vehicle for C1/C3/C4**, not a separate task; it is also what makes the D.1 runner tests possible.
- **C.1 (options table) before E.2 (exclude UI) and B.5** — the exclude editor extends the Options panel, and toggle scoping is trivial once the table exists. C.1 also simplifies B.1 (destructive becomes a table attribute).
- **B.3 (bounded buffers) rides along with A.1** — the line-consumption code is being rewritten anyway.
- **E.1 (config) is easier after C.1** (serialise options via the table) but not blocked by it.
- **Phase 0 items are all independent** of each other and of Phase 1 — ship first.
- **2.1 → 2.2 → 2.3 strictly ordered.** Phase 1 deliberately uses no API absent from ratatui 0.27, so Phase 2 stays a pure migration.
- **U.2 (running-state feedback) depends on A.1** — a spinner and elapsed timer are only meaningful once the loop stays responsive during a run; the 100 ms poll tick already provides the redraw cadence.
- **U.3 rides along with B.4** (same scrolling rewrite). **U.4's destructive attribute comes from the C.1 options table** — land C.1 first. **U.6 shares B.1's modal plumbing.** U.1, U.5, U.7 are independent.
- **Docs get a second small pass at the end of Phase 1**, once the live-progress claim becomes true.

## 5. Acceptance criteria

**Phase 0 done when:**
- A dry-run with DelSrc toggled provably makes zero filesystem changes (regression test + manual check: empty dirs survive).
- A forced panic mid-session returns to a usable shell (no `reset` needed).
- `make test` runs; `cargo build` and `cargo test` pass on Rust 1.81; `cargo tree` shows no tokio.
- Running with an empty source or destination is refused with a clear message.
- Exit codes render as e.g. "23 — partial transfer" instead of `Some(23)`.
- README matches the 10 real options and the 1.81 toolchain.

**Phase 1 done when:**
- The UI redraws and accepts input (including cancel) throughout a large transfer; the progress gauge and log panel update live; a second run cannot start while one is active.
- The user's progress toggles are honoured and field names match the actual rsync flags.
- No deadlock on stderr-heavy runs (verified with a failing rsync producing large stderr).
- Runs with `--delete` or `--remove-source-files` require explicit confirmation.
- Memory stays bounded on million-line output (buffer cap verified by test).
- Insert mode shows a cursor and supports mid-string editing; the command preview never panics on multi-byte paths (tested with `é`/CJK).
- Option key/label/flag defined in exactly one table; exclude patterns editable in the UI and present in the built command; settings persist across restarts.
- `cargo test` passes hermetically (no `$HOME`/cwd dependence) with the new coverage.
- The full UI is usable in an 80×24 terminal; a running transfer is visibly indicated (spinner/elapsed) and its outcome shown as a badge; enabled destructive toggles are visually distinct from safe ones; `?` opens the key-reference overlay; the theme remains legible on a 256-colour terminal.

**Phase 2 done when:**
- The nix environment provides rustc and clippy from the same release (≥ 1.88); `cargo clippy` runs clean (or with an agreed allow-list).
- The app builds and behaves identically on ratatui 0.30 (manual smoke: navigation, toggles, run, cancel, modal, config load/save).
- The Makefile/CI includes the clippy step.
