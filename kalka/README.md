# Kalka

A Qt 6 / PySide6 GUI frontend for Czkawka, with feature parity with the Krokiet (Slint) interface.

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

- **Dark theme** with full Qt stylesheet
- **Two-bar progress display** - current stage + overall progress, matching the Slint frontend
  - Real-time entry and byte counts (e.g., "Calculating hashes: 15,000/25,000 (500 MB/1 GB)")
  - File collection estimation using cached counts from previous scans
  - Elapsed time display
  - Stage step indicators
- **Project icons** - uses the same SVG icons as the Krokiet interface
- **Image preview panel** for duplicate/similar image results
- **Grouped results view** with tree display for duplicate/similar file groups
- **Selection modes** - Select All/None, Invert, Biggest/Smallest, Newest/Oldest, Shortest/Longest Path
- **File actions** - Delete (with trash support), Move/Copy, Hardlink, Symlink, Rename, Clean EXIF
- **Per-tool settings** - all tool-specific options (hash type, similarity thresholds, etc.)
- **Global settings** - directories, filters, cache, thread count
- **Directory management** - included/excluded paths with add/remove buttons
- **Context menus** - right-click to open file or containing folder
- **Settings persistence** via JSON config files
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
cd kalka
pip install -r requirements.txt
```

### 3. Run

```shell
python main.py
```

The application will auto-detect the `czkawka_cli` binary. If it can't find it, configure the path in Settings.

## Architecture

```
kalka/
├── main.py                    # Entry point
├── requirements.txt           # Python dependencies
├── app/
│   ├── main_window.py         # Main window with all panels
│   ├── left_panel.py          # Tool selection sidebar (14 tools)
│   ├── results_view.py        # Results tree with grouping, selection, sorting
│   ├── action_buttons.py      # Scan/Stop/Delete/Move/Save/Sort buttons with icons
│   ├── tool_settings.py       # Per-tool settings (9 tool panels)
│   ├── settings_panel.py      # Global settings (General/Directories/Filters/Preview)
│   ├── progress_widget.py     # Two-bar progress: current stage + overall
│   ├── preview_panel.py       # Image preview panel
│   ├── bottom_panel.py        # Directory management + error display
│   ├── backend.py             # CLI subprocess interface with JSON progress parsing
│   ├── models.py              # Data models, enums, column definitions
│   ├── state.py               # Application state with Qt signals
│   ├── icons.py               # SVG icon resources from Krokiet icon set
│   └── dialogs/               # Delete, Move, Select, Sort, Save, Rename, About
```

### How it works

1. **Scanning**: The app spawns `czkawka_cli` as a subprocess with `--compact-file-to-save` for JSON results and `--json-progress` for real-time progress data on stderr.

2. **Progress**: JSON lines on stderr provide `ProgressData` with stage index, entry counts, byte counts — the same data the Slint frontend gets via crossbeam channels. The progress widget displays two bars (current stage and overall) with percentage, counts, and elapsed time.

3. **Results**: JSON results are parsed and displayed in a tree view with group headers for duplicate/similar file tools.

4. **File operations**: Delete, move, hardlink, symlink, and rename operations are performed directly in Python. EXIF cleaning and extension/name fixing use `czkawka_cli` subcommands.

## LICENSE

MIT
