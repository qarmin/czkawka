# Kalka Development Plan

Issues and improvements identified during code review, prioritized by impact.

## 1. Critical Bugs

### 1.1 Broken Files and Video Optimizer always return "exit with code 2"
- **Problem**: `czkawka_cli` exits with code 2 for these tools, and kalka treats it as an error
- **Root cause**: Likely a mismatch in CLI subcommand arguments or missing required flags
- **Fix**: Debug the exact CLI invocations in `backend.py`, compare against what krokiet sends. Exit code 2 in czkawka_cli typically means "no results found" or argument error — need to handle it gracefully
- **Files**: `app/backend.py`

### 1.2 Empty folders detection doesn't work
- **Problem**: The tool doesn't detect empty folders
- **Root cause**: Likely a parsing issue — `czkawka_cli empty-folders` may return results in a format that `backend.py` doesn't handle
- **Fix**: Run `czkawka_cli empty-folders` manually, inspect JSON output, fix the parser in `backend.py`
- **Files**: `app/backend.py`

### 1.3 Invalid symlinks incorrectly set Destination Path / Type of Error
- **Problem**: Column values are misplaced or missing for invalid symlinks results
- **Root cause**: JSON field mapping in `backend.py` doesn't match the CLI output for symlinks
- **Fix**: Parse the symlink-specific fields (`destination`, `error_type`) from CLI JSON and map to the correct columns
- **Files**: `app/backend.py`, `app/models.py` (TAB_COLUMNS)

### 1.4 Random "Error: czkawka_cli exited with code 2" errors
- **Problem**: Opaque error messages with no useful context
- **Fix**:
  - Capture and display stderr from czkawka_cli
  - Map known exit codes to human-readable messages
  - Show the actual CLI command that failed (for debugging)
  - Don't treat exit code 2 as a fatal error if results were produced
- **Files**: `app/backend.py`, `app/main_window.py`

## 2. UX / Behavior Issues

### 2.1 Scan/Stop button can be spammed
- **Problem**: No state management — button works in 2 modes with no "disabled while processing" state
- **Fix**:
  - Add a `STOPPING` state to the scan lifecycle
  - Disable both Scan and Stop buttons during the transition period
  - Show "Stopping scan, please wait..." in status bar
  - Re-enable Scan only after the process is fully terminated
- **Files**: `app/action_buttons.py`, `app/main_window.py`, `app/state.py`

### 2.2 Double-click opens image preview instead of expected behavior
- **Problem**: Double-clicking a result shows image preview — users expect it to open the file or select it
- **Fix**: Change double-click to open the file in the system file manager (like krokiet). Move preview to single-click selection or a dedicated preview button
- **Files**: `app/results_view.py`, `app/main_window.py`

### 2.3 Cross-tab result loading allows loading incompatible results
- **Problem**: Loading results saved from one tool (e.g., big files) into another (e.g., duplicate files) — data is lost or columns don't match
- **Fix**:
  - Save the tool type (ActiveTab) in the JSON results file
  - On load, validate that results match the current tab or auto-switch to the correct tab
  - Show a warning if there's a mismatch
- **Files**: `app/dialogs/save_dialog.py`, `app/main_window.py`

### 2.4 Error/warning panel is never cleared
- **Problem**: Bottom panel always shows "Error: czkawka_cli exited with code 2", never shows useful krokiet-style messages
- **Fix**:
  - Clear the error panel when starting a new scan
  - Show per-scan warnings/info (like krokiet does)
  - Append new errors instead of replacing
  - Add a "Clear" button
- **Files**: `app/bottom_panel.py`, `app/main_window.py`

### 2.5 Duplicated tab info at the bottom
- **Problem**: Bottom panel shows the same info as the selected tab
- **Fix**: Remove the redundant display or repurpose the bottom area for directories only (its original purpose)
- **Files**: `app/bottom_panel.py`, `app/main_window.py`

### 2.6 Bad Names missing "new file name" column
- **Problem**: Has "Error Type" column but doesn't show what the corrected name would be
- **Fix**: Parse the `new_name` field from CLI output and add a "New Name" column to the Bad Names tab
- **Files**: `app/models.py` (TAB_COLUMNS), `app/backend.py`

## 3. Performance Issues

### 3.1 GUI is slow with large result sets (20,000+ entries)
- **Problem**: Tab switching takes 0.5-several seconds, sorting is slow
- **Root cause**: `QTreeWidget` rebuilds all items on every operation — O(n) item creation
- **Fix** (incremental):
  1. **Short term**: Use `blockSignals()` during bulk operations (already partially done), add batch item creation
  2. **Medium term**: Switch from `QTreeWidget` to `QTreeView` + `QAbstractItemModel` — this enables virtual rendering (only visible items are rendered)
  3. **Long term**: Implement lazy loading / pagination for very large result sets
- **Files**: `app/results_view.py`

### 3.2 JSON results are huge
- **Problem**: Each file entry stores all data from all possible modes — much larger than necessary
- **Fix**:
  - Only serialize fields relevant to the current tool mode
  - Use a compact format: skip null/empty fields
  - Consider storing `__size_bytes` and `__modified_date_ts` as primary fields and deriving display strings
- **Files**: `app/dialogs/save_dialog.py`, `app/backend.py`

## 4. Visual / Polish

### 4.1 Modernize the look
- **Problem**: Current look is a mix of GTK/Slint versions, doesn't leverage PySide6/Qt styling
- **Fix**:
  - Use the Kalka logo (already done, `icons/kalka.png`)
  - Keep the system theme approach (KDE Breeze/Adwaita) but polish spacing, margins, and layout
  - Consider adding subtle styling touches (rounded group boxes, consistent icon sizes, toolbar styling)
  - Ensure dark/light theme consistency
- **Files**: `app/main_window.py` (`_apply_theme`), all panel files

### 4.2 Non-ASCII character handling causes display issues
- **Problem**: Resizing columns causes strange jumps in text with emoji/special characters
- **Root cause**: Qt font metrics miscalculate width for certain Unicode characters
- **Fix**:
  - Set a font that has good Unicode coverage
  - Use `QHeaderView.ResizeToContents` for affected columns
  - Consider eliding long paths instead of letting them cause layout thrash
- **Files**: `app/results_view.py`

### 4.3 File size filter defaults are unclear
- **Problem**: Not clear what the default min/max file size values are
- **Fix**:
  - Show default values in placeholder text (e.g., "Default: 8192 bytes (8 KB)")
  - Or set explicit default values and show them
- **Files**: `app/settings_panel.py`, `app/models.py`

## 5. Missing Features

### 5.1 No logging to terminal
- **Problem**: No terminal output makes debugging very difficult
- **Fix**:
  - Add Python `logging` module throughout the codebase
  - Log CLI commands, exit codes, stderr output, timing
  - Add `--verbose` / `--debug` CLI flags to main.py
  - Log to both terminal and an in-app log panel
- **Files**: all files, `main.py`

## 6. New Features (inspired by Fast Duplicate File Finder & dupeGuru)

### 6.1 Reference/Source directory protection
- **Problem**: No way to mark directories as "originals" that should never be selected for deletion — only used for comparison
- **Inspiration**: Both FDFF and dupeGuru have this concept ("source folders" / "reference directories")
- **Fix**:
  - Add a third directory category: "Reference" (alongside Included/Excluded)
  - Files in reference directories participate in duplicate detection but are never auto-selected for deletion
  - Add `--reference-dir` flag to `czkawka_cli` and propagate through `czkawka_core`
  - In kalka, add a "Reference" section to the bottom panel directory management
- **Files**: `czkawka_core`, `czkawka_cli`, `app/bottom_panel.py`, `app/backend.py`, `app/models.py`

### 6.2 Fuzzy filename matching for duplicates
- **Problem**: Duplicate detection only supports exact name or size+name — misses renamed duplicates like `report_final.pdf` vs `report_final_v2.pdf`
- **Inspiration**: FDFF has configurable similarity threshold (0–100%) for filenames
- **Fix**:
  - Add a fuzzy name matching mode to `czkawka_core` duplicate finder using Levenshtein or Jaro-Winkler distance
  - Expose as a new `--search-method fuzzy-name` option in `czkawka_cli` with `--name-similarity-threshold` parameter
  - Add UI controls in kalka's duplicate tool settings
- **Files**: `czkawka_core` (duplicate finder), `czkawka_cli`, `app/tool_settings.py`, `app/backend.py`

### 6.3 Extended file preview (PDF, text, video thumbnails)
- **Problem**: Preview panel only supports images (JPG, PNG, GIF, BMP, WebP, TIFF, ICO)
- **Inspiration**: FDFF previews PDFs, Excel, TXT, video, audio, and 300+ RAW camera formats
- **Fix**:
  - Add PDF preview using `QPdfDocument` (available in PySide6)
  - Add plain text preview for TXT, CSV, JSON, XML, etc.
  - Add video thumbnail preview using first-frame extraction (via `QMediaPlayer` or ffmpeg subprocess)
  - Consider RAW image support via `rawpy` or similar library
- **Files**: `app/preview_panel.py`, `requirements.txt`

### 6.4 Combined selection criteria
- **Problem**: Select dialog offers individual modes (biggest, newest, shortest path) but no way to combine them
- **Inspiration**: FDFF's "Quick Check/Uncheck" dialog combines multiple criteria (date + size + path length)
- **Fix**:
  - Redesign the select dialog to allow combining criteria with AND/OR logic
  - E.g., "keep newest AND shortest path" or "select biggest OR oldest"
  - Add priority ordering when criteria conflict
- **Files**: `app/dialogs/select_dialog.py`

### 6.5 Side-by-side comparison view
- **Problem**: No way to visually compare two files from a duplicate/similar group
- **Inspiration**: dupeGuru's picture mode and FDFF's file preview
- **Fix**:
  - Add a split-view mode to the preview panel showing two selected files side by side
  - For images: show both images with zoom sync
  - For text files: show a diff view
  - Activate when two items are selected in a results group
- **Files**: `app/preview_panel.py`, `app/results_view.py`

### 6.6 CSV/JSON export of results
- **Problem**: Results can only be saved/loaded in kalka's internal format
- **Inspiration**: FDFF exports to XML, CSV, and proprietary formats
- **Fix**:
  - Add export options: CSV, JSON, and plain text
  - Include column headers and group separators
  - Add "Export" button to action bar or extend existing Save dialog
- **Files**: `app/dialogs/save_dialog.py`, `app/action_buttons.py`

### 6.7 Exclude from self-scan per folder
- **Problem**: No way to prevent files within a folder from being compared against each other
- **Inspiration**: FDFF's "Exclude from self-scan" prevents internal comparison within a folder — only compares against files in other folders
- **Fix**:
  - Add `--no-self-compare` flag per directory in `czkawka_core` and `czkawka_cli`
  - Useful for comparing a "known good" archive against a messy downloads folder
  - Add a per-directory toggle in kalka's bottom panel
- **Files**: `czkawka_core`, `czkawka_cli`, `app/bottom_panel.py`, `app/backend.py`

### 6.8 Similarity confidence scores in results
- **Problem**: Similar image/video/music results don't show how similar files are to each other
- **Inspiration**: Both FDFF and dupeGuru show similarity percentages
- **Fix**:
  - `czkawka_core` already computes similarity internally — expose the score in JSON output
  - Add a "Similarity" column to similar images/videos/music result tabs
  - Color-code or sort by confidence to help users prioritize
- **Files**: `czkawka_core`, `czkawka_cli`, `app/models.py` (TAB_COLUMNS), `app/backend.py`, `app/results_view.py`

### 6.9 Scan profiles/presets
- **Problem**: Users who run recurring cleanup tasks must reconfigure settings each time
- **Fix**:
  - Save/load entire scan configurations (tool + settings + directories) as named profiles
  - Store profiles as JSON files in the config directory
  - Add a profile selector dropdown in the UI
  - Useful for automation: `czkawka_cli --profile weekly-cleanup`
- **Files**: `czkawka_cli`, `app/settings_panel.py`, `app/main_window.py`

### 6.10 Idle-priority scanning
- **Problem**: Scanning large drives can slow down other applications
- **Inspiration**: FDFF offers IDLE process priority settings
- **Fix**:
  - Add a "Low priority" toggle in kalka settings
  - When enabled, spawn `czkawka_cli` with `nice -n 19` and `ionice -c 3` on Linux
  - On Windows, use `IDLE_PRIORITY_CLASS`
- **Files**: `app/backend.py`, `app/settings_panel.py`

### 6.11 Fuzzy music tag matching
- **Problem**: Similar music tool matches by exact tags or fingerprint — misses variations like "Beatles" vs "The Beatles"
- **Inspiration**: dupeGuru's music mode does fuzzy matching on tags
- **Fix**:
  - Add fuzzy string comparison for music tags in `czkawka_core`
  - Normalize common patterns: strip "The ", case-insensitive, trim whitespace
  - Expose as `--tag-match-mode exact|fuzzy` in `czkawka_cli`
- **Files**: `czkawka_core` (music duplicate finder), `czkawka_cli`, `app/tool_settings.py`

### 6.12 Similar document/archive content detection
- **Problem**: No way to find documents with similar but not identical content
- **Inspiration**: FDFF analyzes file content to find similar documents even with rearranged paragraphs
- **Fix**:
  - Add a new tool or mode in `czkawka_core` that extracts text from documents (PDF, DOCX, TXT) and computes similarity using shingling/MinHash or similar algorithm
  - Extend to archives: compare file listings within ZIP/TAR files
  - This is a large feature — consider as a separate tool alongside existing ones
- **Files**: `czkawka_core` (new module), `czkawka_cli`, `app/models.py`, `app/backend.py`

### 6.13 Drag & drop directory addition
- **Problem**: Adding directories requires using a file dialog or typing paths
- **Fix**:
  - Enable drag & drop on the bottom panel's directory lists
  - Accept folder drops from the system file manager
  - Visual feedback (highlight) when dragging over the directory area
- **Files**: `app/bottom_panel.py`

## Priority Order

| Priority | Item | Effort | Where |
|----------|------|--------|-------|
| P0 | 1.1 Broken Files / Video Optimizer exit code | Small | kalka |
| P0 | 1.4 Opaque error messages | Small | kalka |
| P0 | 2.1 Scan/Stop button state management | Small | kalka |
| P1 | 1.2 Empty folders detection | Small | kalka |
| P1 | 1.3 Invalid symlinks columns | Small | kalka |
| P1 | 5.1 Terminal logging | Medium | kalka |
| P1 | 2.3 Cross-tab result loading | Medium | kalka |
| P1 | 2.4 Error panel improvements | Small | kalka |
| P1 | 6.1 Reference/source directory protection | Medium | czkawka_core + czkawka_cli + kalka |
| P1 | 6.2 Fuzzy filename matching | Medium | czkawka_core + czkawka_cli + kalka |
| P1 | 6.5 Side-by-side comparison view | Medium | kalka |
| P1 | 6.6 CSV/JSON export | Small | kalka |
| P2 | 3.1 GUI performance with large results | Large | kalka |
| P2 | 2.2 Double-click behavior | Small | kalka |
| P2 | 2.6 Bad Names new name column | Small | kalka |
| P2 | 4.3 File size filter defaults | Small | kalka |
| P2 | 6.3 Extended file preview (PDF, text, video) | Medium | kalka |
| P2 | 6.4 Combined selection criteria | Small | kalka |
| P2 | 6.7 Exclude from self-scan per folder | Medium | czkawka_core + czkawka_cli + kalka |
| P2 | 6.8 Similarity confidence scores | Small | czkawka_core + czkawka_cli + kalka |
| P2 | 6.9 Scan profiles/presets | Small | czkawka_cli + kalka |
| P2 | 6.13 Drag & drop directory addition | Small | kalka |
| P3 | 3.2 Compact JSON results | Medium | kalka |
| P3 | 4.1 Modern look polish | Medium | kalka |
| P3 | 4.2 Non-ASCII display issues | Medium | kalka |
| P3 | 2.5 Duplicated tab info | Small | kalka |
| P3 | 6.10 Idle-priority scanning | Small | kalka |
| P3 | 6.11 Fuzzy music tag matching | Medium | czkawka_core + czkawka_cli |
| P3 | 6.12 Similar document/archive content | Large | czkawka_core + czkawka_cli + kalka |
