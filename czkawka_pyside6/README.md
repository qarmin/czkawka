# Czkawka PySide6

A Qt 6 / PySide6 GUI frontend for Czkawka, with feature parity with the Krokiet (Slint) interface. KDE6/Plasma compliant (14/14 checks passed).

This frontend uses `czkawka_cli` as its backend, communicating via JSON output for results and `--json-progress` for real-time progress data.

## Features

All 14 scanning tools are supported:

- Duplicate Files (by hash, size, name, or size+name)
- Empty Folders / Empty Files
- Big Files (biggest or smallest)
- Temporary Files
- Similar Images (with configurable hash algorithm, size, similarity)
- Similar Videos (with crop detection, skip forward, duration)
- Similar Music (by tags or audio fingerprint)
- Invalid Symlinks
- Broken Files (audio, PDF, archive, image, video)
- Bad Extensions
- Bad Names (uppercase, emoji, spaces, non-ASCII, restricted charset)
- EXIF Remover
- Video Optimizer (crop black bars / transcode)

### GUI features

- **KDE6/Plasma compliant** - inherits system theme (Breeze, Adwaita, etc.), XDG icons via `QIcon.fromTheme()`, `QStandardPaths`, `.desktop` file, AppStream metadata
- **Two-bar progress display** - current stage + overall progress with stage index (e.g., "[3/7] Calculating prehashes")
  - Real-time entry and byte counts (e.g., "50,000/94,500 (308 MB/371 MB)")
  - File collection estimation using cached counts from previous scans
  - Elapsed time display
- **Resizable and sortable columns** - drag header edges to resize, click to sort (ascending/descending toggle), numeric columns sort by value
- **Filter/search bar** - filter results by filename or path in real-time
- **Column visibility toggle** - right-click column header to show/hide columns
- **Keyboard shortcuts**:
  - `Ctrl+S` / `F5` — Start scan
  - `Escape` — Stop scan
  - `Ctrl+A` — Select all
  - `Ctrl+I` — Invert selection
  - `Ctrl+D` — Delete selected
  - `Ctrl+M` — Move selected
  - `Ctrl+Shift+S` — Save results
  - `Ctrl+L` — Load results
- **Drag-and-drop** - drop directories onto the bottom panel to add include paths
- **System tray** - minimize to tray, scan completion notifications
- **Scan history** - persistent log of past scans with timestamp, tool, entries found, duration
- **Scan queue** - queue multiple scan types and run them sequentially
- **Diff view** - side-by-side file comparison for duplicates (right-click two selected files)
- **Image preview panel** for duplicate/similar image results
- **Grouped results view** with spanning group headers for duplicate/similar file groups
- **Selection modes** - Select All/None, Invert, Biggest/Smallest, Newest/Oldest, Shortest/Longest Path
- **File actions** - Delete (with trash support), Move/Copy, Hardlink, Symlink, Rename, Clean EXIF
- **Save/Load results** - save as JSON, text, or CSV with task-specific filenames; load previously saved results (supports both app and raw CLI JSON formats)
- **Per-tool settings** - all tool-specific options (hash type, similarity thresholds, etc.)
- **Global settings** - directories, filters, cache, thread count
- **Directory management** - included/excluded paths with add/remove buttons and drag-and-drop
- **Context menus** - right-click to open file, open folder, select/deselect, compare
- **Settings persistence** - window geometry, splitter positions, and settings saved via JSON
- **Auto-detection** of `czkawka_cli` binary (checks PATH, cargo target directory, cargo metadata)

## Requirements

- Python 3.10+
- PySide6 >= 6.6.0
- `czkawka_cli` binary (installed or in PATH)
- Optional: `send2trash` (for trash support on Linux)
- Optional: `Pillow` (for EXIF cleaning fallback)

## Installation

### 1. Install czkawka_cli

```shell
# From the project root
cargo install --path czkawka_cli

# Or build it
cargo build --release -p czkawka_cli
```

### 2. Install Python dependencies

```shell
cd czkawka_pyside6
pip install -r requirements.txt
```

### 3. Run

```shell
python main.py
```

The application will auto-detect the `czkawka_cli` binary. If it can't find it, configure the path in Settings.

## Testing

The project includes a comprehensive test suite (98 tests):

```shell
cd czkawka_pyside6
pip install pytest
QT_QPA_PLATFORM=offscreen python -m pytest tests/ -v
```

### Test coverage

| Test file | Tests | Coverage |
|---|---|---|
| `test_models.py` | 16 | Enums, data models, settings defaults, column definitions |
| `test_backend.py` | 25 | CLI command building (all 14 tools), JSON parsing (flat/grouped/empty), formatters |
| `test_widgets.py` | 24 | ResultsView (selection, sorting, filtering), LeftPanel, ActionButtons, ProgressWidget |
| `test_new_features.py` | 20 | ScanHistory, ScanQueue, SaveLoad roundtrip, DiffDialog |
| `test_main_window.py` | 13 | Integration: window creation, all tabs, state, features presence |
| **Total** | **98** | **All pass** |

## Architecture

```
czkawka_pyside6/
├── main.py                    # Entry point
├── requirements.txt           # Python dependencies
├── tests/                     # Test suite (98 tests)
│   ├── conftest.py            # Qt app fixture
│   ├── test_models.py         # Data model tests
│   ├── test_backend.py        # CLI interface tests
│   ├── test_widgets.py        # Widget component tests
│   ├── test_new_features.py   # Feature-specific tests
│   └── test_main_window.py    # Integration tests
├── app/
│   ├── main_window.py         # Main window with all panels
│   ├── left_panel.py          # Tool selection sidebar (14 tools)
│   ├── results_view.py        # Results tree with grouping, selection, sorting, filtering
│   ├── action_buttons.py      # Scan/Stop/Delete/Move/Save/Load/Sort buttons with icons
│   ├── tool_settings.py       # Per-tool settings (9 tool panels)
│   ├── settings_panel.py      # Global settings (General/Directories/Filters/Preview)
│   ├── progress_widget.py     # Two-bar progress: current stage + overall
│   ├── preview_panel.py       # Image preview panel
│   ├── bottom_panel.py        # Directory management + error display (with drag-and-drop)
│   ├── backend.py             # CLI subprocess interface with JSON progress parsing
│   ├── models.py              # Data models, enums, column definitions
│   ├── state.py               # Application state with Qt signals, geometry persistence
│   ├── icons.py               # XDG theme icons with SVG fallbacks from Krokiet
│   ├── system_tray.py         # System tray integration with notifications
│   ├── scan_history.py        # Persistent scan history log
│   ├── scan_queue.py          # Multi-tab sequential scan queue
│   └── dialogs/               # Delete, Move, Select, Sort, Save/Load, Rename, About, Diff
```

### How it works

1. **Scanning**: The app spawns `czkawka_cli` as a subprocess with `--compact-file-to-save` for JSON results and `--json-progress` for real-time progress data on stderr.

2. **Progress**: JSON lines on stderr provide `ProgressData` with stage index, entry counts, byte counts — the same data the Slint frontend gets via crossbeam channels. The progress widget displays two bars (current stage and overall) with percentage, counts, and elapsed time.

3. **Results**: JSON results are parsed and displayed in a tree view with spanning group headers for duplicate/similar file tools. Columns are resizable and sortable (click header), with a filter bar for real-time search.

4. **File operations**: Delete, move, hardlink, symlink, and rename operations are performed directly in Python. EXIF cleaning and extension/name fixing use `czkawka_cli` subcommands.

5. **Persistence**: Window geometry, scan estimates, scan history, and settings are saved via `QStandardPaths` for XDG compliance.

### KDE6/Plasma Compliance

The app passes all 14 KDE compliance checks:

- System theme inherited (no color overrides)
- `QIcon.fromTheme()` with standard XDG icon names + SVG fallbacks
- `.desktop` file and AppStream `metainfo.xml`
- `QStandardPaths` for config/cache paths
- `desktopFileName` and `organizationDomain` set
- System font inherited, HiDPI via Qt6 native support
- System dialog icons via `style().standardIcon()`

## LICENSE

MIT
