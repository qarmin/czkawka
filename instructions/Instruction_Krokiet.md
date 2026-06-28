# Krokiet - Instructions

## Table of Contents

- [Glossary](#glossary)
- [Installation](#installation)
- [Interface Overview](#interface-overview)
- [Tool-by-Tool Guide](#tool-by-tool-guide)
- [Settings](#settings)
- [Common Workflows](#common-workflows)
- [Config and Cache Files](#config-and-cache-files)
- [Tips and Tricks](#tips-and-tricks)

---

## Glossary

For shared terminology (Reference paths, Included/Excluded paths, Excluded items, Cache) see [Terminology in the main guide](Instruction.md#terminology-shared-across-all-frontends).

Krokiet-specific or tool-focused terms:

| Term | Definition |
|------|------------|
| **Perceptual hash** | A hash computed from visual or audio content, designed so similar content produces similar hashes. Used for similar images, videos, and music fingerprint mode. |
| **Prehash** | A fast partial hash of the beginning and end of a file. Used to quickly rule out non-duplicates before computing the full hash, speeding up large scans. |
| **Hash group** | A set of files sharing the same hash (or visually similar within the chosen threshold) - candidate duplicates. |
| **Similarity threshold** | For similar images/videos/music: the maximum allowed difference between two items for them to be considered similar. Lower = stricter. |
| **Preset** | A saved configuration profile storing scan directories, filters, and tool parameters. Multiple presets can be created and switched between. |
| **Backend / renderer** | The graphics API used to draw the Krokiet window. Options: femtovg (OpenGL, default), skia, software. Use `software` on unusual setups or when OpenGL is unavailable. |
| **Hard link** | A filesystem feature where two filenames point to the same inode. Krokiet detects hard links and counts them only once by default to avoid false duplicates. |

---

## Installation

Prebuilt binaries (no extra dependencies required) are available for Windows 10/11, macOS, and Ubuntu 22.04+.

Download from the [GitHub releases page](https://github.com/qarmin/czkawka/releases).

For the **Similar Videos** tool, **ffmpeg** must be installed separately:

| OS | Command |
|----|---------|
| Linux | `sudo apt install ffmpeg` |
| macOS | `brew install ffmpeg` |
| Windows | `choco install ffmpeg` or download from ffmpeg.org and place `ffmpeg.exe` in `PATH` |

### Optional features (self-compilation only)

| Feature | Library | Purpose |
|---------|---------|---------|
| `heif` | `libheif` | HEIF/HEIC image support |
| `libraw` | `libraw` | RAW camera image support |
| `libavif` | `libavif`, `libdav1d` | AVIF image support |

### Compile from source

```shell
# Simplest - installs the latest optimized binary
cargo install krokiet --locked

# With optional image format support
cargo build --release --bin krokiet --features "heif,libraw,libavif"
```

### Alternative renderers

By default femtovg (OpenGL) and software renderers are built in. Switch at runtime:

```shell
SLINT_BACKEND=software ./krokiet        # pure software rendering, safest fallback
SLINT_BACKEND=winit-femtovg ./krokiet  # OpenGL via femtovg (default)
SLINT_BACKEND=skia ./krokiet           # Skia (requires winit_skia_opengl compile feature)
```

To verify which backend is active:

```shell
SLINT_DEBUG_PERFORMANCE=refresh_lazy,console,overlay ./krokiet
# Prints: "Slint: Build config: release; Backend: femtovg"
```

---

## Interface Overview

<img width="600" alt="Screenshot From 2026-06-28 07-41-46" src="https://github.com/user-attachments/assets/2db13c50-3866-4d55-bbfc-75b55db6972e" />

The main window has six areas:

| # | Area | Purpose |
|---|------|---------|
| 1 | **Left panel** | Tool selector - logo at top, 14 tools listed vertically, Settings gear at the very bottom |
| 2 | **Results area** | Scan results displayed as grouped rows; occupies the bulk of window height |
| 3 | **Action bar** | Below the results area - Scan/Stop buttons plus selection helpers and action buttons (delete, move, rename, hardlink, ...). Right side has two toggle buttons: folder icon (show/hide directory panel) and info icon (show/hide error log). |
| 4 | **Status bar** | Immediately below the action bar - read-only text line showing current scan state or result summary ("Found N items in Xms"). |
| 5 | **Directory panel** | Togglable panel below the status bar (folder icon button). Shows Included Paths (left) and Excluded Paths (right). Extensions, size filters, and excluded items are in Settings, not here. |
| 6 | **Right pane** | Optional image preview / tool sub-settings panel; shown for Similar Images and other visual tools when preview or sub-settings are active. |

### Left panel - tool list

Available tools: Duplicate Files, Empty Folders, Big Files, Empty Files, Temporary Files, Similar Images, Similar Videos, Same Music, Invalid Symlinks, Broken Files, Bad Extensions, Bad Names, Exif Remover, Video Optimizer.

### Directory panel

Toggled by the folder icon button on the right side of the action bar. Contains only:

- **Included paths** - directories that will be scanned. Each row has a "Ref" checkbox to mark it as a reference path (protected from deletion). Add paths with the folder/file/manual-entry buttons at the top.
- **Excluded paths** - directories that will be ignored during scanning. Add with the same buttons.

Extensions, size filters, excluded items, and recursive search are configured in **Settings**, not in this panel.

### Results area

- **Header rows** (bold) represent groups (e.g., a duplicate group or a set of similar images).
- **File rows** show name, path, size, and tool-specific extra info (e.g., image dimensions, similarity score).
- Left-click a row to toggle its selection (also reverses focus for preview). Ctrl+click reverses individual selection without changing others; Shift+click selects a range.
- Double left-click to open the file in the default application.
- Right-click opens a context menu: Open Item, Open Parent Folder, Remove from Results, Remove All from Folder, Remove All from Folder (recursive), Select All from Folder, Select All from Folder (recursive), Exclude Parent Folder, Exclude Item, Copy File Name, Copy Parent Folder, Copy Full Path, Rename.

### Action bar

The action bar sits below the results area. Buttons visible depend on the active tool and whether results exist.

**Always present (when not scanning):**
- **Scan** - start the scan for the active tool
- **Select** - open a popup with selection presets: Select All, Unselect All, One oldest/newest/biggest/smallest (size or resolution), Except oldest/newest/biggest/smallest (size or resolution), Select shortest/longest path, Invert selection, Invert selection in group, Custom (filter by column values)
- **Move** - move selected files to a chosen directory
- **Delete** - permanently delete selected files
- **Trash** - move selected files to system trash (recoverable)
- **Save** - export results to a file
- **Sort** - open sort options popup

**Tool-specific buttons (only shown for relevant tool):**
- **Rename** - apply suggested fix to file name (Bad Extensions and Bad Names)
- **Optimize** - transcode / crop selected videos (Video Optimizer)
- **Clean** - strip EXIF metadata from selected files (Exif Remover)
- **Compare** - open side-by-side image comparison overlay (Similar Images, when results are present)
- **Hardlink** / **Softlink** - replace duplicates with hard links or symbolic links (tools that return grouped results)

---

## Tool-by-Tool Guide

### Duplicate Files

Finds files with identical content (or matching name/size, depending on the method chosen).

**Sub-settings (right pane):**

Check method (UI order, Hash is default):

| Method | Notes |
|--------|-------|
| Hash | Compares full file content via hash; recommended; default |
| Size | Compares file size only; many false positives |
| Name | Compares filename only; many false positives |
| Size and Name | Combines size and name comparison |

Hash type (used with Hash method):

| Hash | Notes |
|------|-------|
| Blake3 | Default |
| CRC32 | |
| XXH3 | |

Case sensitive - toggle for name-based methods only.

**In global Settings (under "Duplicate Files" section):**
- Image preview - show thumbnail for selected duplicate
- Minimal size of cached files - Hash (KB) - minimum file size to cache hash for
- Use prehash - cache partial hashes (first/last bytes) to skip non-duplicates faster
- Minimal size of cached files - Prehash (KB)

### Empty Folders

Finds directories that contain no files or subdirectories. Uses a recursive algorithm that propagates "not empty" status upward through the directory tree. No sub-settings.

### Big Files

Finds the N largest (or N smallest) files in the specified directories.

**Sub-settings (right pane):**
- Method: The Biggest / The Smallest
- Number of files (default 50)

### Empty Files

Finds files with 0 bytes.

**Sub-settings (right pane) - additional file types to find:**
- **Files filled with null bytes** - also find non-empty files whose entire content is null bytes (0x00); disabled by default
- **Files filled with non-printable characters** - also find files consisting only of non-printable ASCII characters (null, space, tab, CR, LF, VT, FF); disabled by default

### Temporary Files

Finds files matching common temporary file patterns.

**Sub-settings (right pane):**
Editable extension/pattern list, with a Reset button. Default list:
`#,thumbs.db,.bak,~,.tmp,.temp,.ds_store,.crdownload,.part,.cache,.dmp,.download,.partial`

### Similar Images

Finds images that look alike but are not byte-for-byte identical (different resolution, watermarks, compression artifacts, JPEG re-save, etc.).

**Sub-settings (right pane):**

| Setting | Default | Notes |
|---------|---------|-------|
| Hash size | 16 | Options: 8, 16, 32, 64. Higher = more precise; requires higher max difference threshold. |
| Resize algorithm | Lanczos3 | Options: Lanczos3, Nearest, Triangle, Gaussian, CatmullRom. |
| Hash type | Mean | Options: Mean, Gradient, BlockHash, VertGradient, DoubleGradient, Median. BlockHash does not resize before hashing. |
| Geometric invariance | Off | Options: Off, Mirror + Flip, Mirror + Flip + Rotate 90. |
| Ignore images with same size | off | Skip groups where all images have identical byte size. |
| Ignore images with same resolution | off | Skip groups where all images have identical pixel dimensions. |
| Max difference | 10 | Max hash distance. Raise for more matches, lower for stricter. |

**In global Settings (under "Similar Images tool" section):**
- Image preview - show thumbnail for selected image

### Similar Videos

Finds visually or acoustically similar videos. Requires **ffmpeg** installed.

**Sub-settings (right pane):**

Toggle at the top: **Compare by audio fingerprint** - switches between visual and audio comparison mode.

**Visual mode** (default):
- Quick preset: Custom / Near-identical / Similar / Movies (long skip). Non-custom presets set the sliders below automatically.
- When Custom: Max difference (0-20, default 15), Skip duration [s], Video hash duration, Window count (default 5), Duration tolerance [%] (default 20), Min matching windows (default 60%), Subclip min match
- Letterbox crop detect (Custom only)
- Ignore videos with same size
- Ignore videos with same resolution

**Audio fingerprint mode:**
- Quick preset: Custom / Identical videos / Clip in longer video / Similar content
- When Custom: Similarity [%], Min length ratio (shorter/longer), Min file duration [s], Max audio difference
- Ignore videos with same size
- Ignore videos with same resolution

**In global Settings (under "Similar Videos tool" and "Video Thumbnails" sections):**
- Generate thumbnails - enable thumbnail generation for results preview
- Thumbnail position in video (%)
- Generate thumbnail grid instead of single image
- Number of tiles per side in thumbnail grid
- Delete unused video thumbnails older than 7 days at app startup

### Same Music

Finds duplicate or similar music files.

**Sub-settings (right pane):**

Audio check type: **Tags** (default) or **Fingerprint**.

When Tags:
- Approximate Tag Comparison toggle
- Compared tags (individually toggleable): Title (on), Artist (on), Bitrate (off), Genre (off), Year (off), Length (off)

When Fingerprint:
- Compare within groups of similar titles toggle
- Max difference slider
- Minimal fragment duration slider

### Invalid Symlinks

Finds symbolic links whose target does not exist or that form a circular chain. No sub-settings.

### Broken Files

Finds files that fail to open with their expected library (corrupted or truncated content).

**Sub-settings (right pane) - file types to check (defaults shown):**
- Audio - on by default
- PDF - off
- Archive - off
- Image - off
- Video (ffprobe) - off; quick header check, requires ffmpeg
- Video (ffmpeg) - off; full decode, very slow, requires ffmpeg
- Font - on by default
- Markup (JSON/XML/TOML) - on by default

Note: false positives can occur depending on the library used. Always verify before deleting.

### Bad Extensions

Finds files whose content (detected from magic bytes) does not match their extension. No sub-settings.

The "Proper Extension" column shows the detected type and compatible extensions. Use the **Rename** button to apply the suggested fix.

### Bad Names

Finds files with problematic names.

**Sub-settings (right pane) - checks (defaults shown):**
- Uppercase extension (e.g. `.JPG`) - on by default
- Emoji in name - on by default
- Leading/trailing spaces - on by default
- Non-ASCII chars (e.g. ą, ć, ñ) - on by default; suggests ASCII equivalents
- Limited charset - off; transliterates to ASCII then flags chars outside 0-9a-zA-Z plus user-defined allowed chars; when enabled, an "Allowed chars" text field appears (default `_- `)
- Duplicated chars (e.g. `file---name`) - off; flags consecutive duplicated non-alphanumeric characters

Results include a suggested corrected name.

### Exif Remover

Finds image files containing EXIF metadata (GPS coordinates, camera model, creation date, etc.).

**Sub-settings (right pane):**
- Ignored tags - comma-separated list of EXIF tag names to preserve (leave empty to strip all tags)

Use the **Clean** button to remove EXIF from selected files.

### Video Optimizer

Identifies videos suitable for optimization.

**Sub-settings (right pane):**

Mode: **Crop** (default) or **Transcode**.

When Crop:
- Crop type: Black Bars (default) / Static Content
- Black pixel threshold
- Black bar min percentage

When Transcode:
- Excluded codecs - codecs to skip (default: `h265,av1,vp9`; Reset button available)

Use the **Optimize** button to process selected videos (opens a confirmation popup with output settings).

---

## Settings

Open settings via the gear icon at the bottom of the left panel. Settings is a single scrollable list with section headers - not a tabbed view.

**Scan filters** (these are here, not in the directory panel):
- **Allowed extensions** - leave empty to scan all types, or enter comma-separated values (e.g. `jpg,png`). Supports keywords: `image`, `video`, `text` (expanded to the relevant extension lists).
- **Excluded extensions** - extensions to skip
- **Excluded items** - glob patterns filtered out (e.g. `*/tmp*`, `*/.git`)
- **File size (KB)** - min and max file size filter
- **Recursive search** - toggle whether subdirectories are scanned

**General:**
- **Language** - UI language; changes apply immediately, no restart needed
- **Dark theme** - toggle dark/light theme; applies immediately
- **Show only icons** - hide button labels in the action bar
- **Hide hard links** - count hard-linked files only once in results

**Performance:**
- **Thread number** - CPU threads to use; requires restart to take effect
- **Use cache** - enable/disable hash and thumbnail cache
- **Use prehash** (Duplicate Files section) - cache partial hashes of large files to speed up re-scans
- **Delete automatically outdated cache entries** - auto-clean stale cache records (verified at most once per week)
- **Application scale** - manual UI scale factor; requires restart to take effect

### Presets

Each preset stores: included/excluded paths, extension and size filters, tool-specific parameters.

Presets are saved at `~/.config/krokiet/config_preset_N.json` (Linux).

---

## Common Workflows

### Removing duplicates while protecting a backup

1. Add your working folder to **Included paths**
2. Add your backup folder to **Included paths**
3. Check the **Ref** checkbox in the backup folder's row in the Included Paths list
4. Select **Duplicate Files** tool, set method **Hash**, hash type **Blake3**
5. Click **Scan**
6. Click **Select** → choose "Except newest" (or whichever strategy fits)
7. Verify that no reference-path files are selected
8. Click **Trash** or **Delete**

### Cleaning up a photo library (similar images)

1. Add all photo directories to **Included paths**
2. Select **Similar Images** tool
3. Set similarity **10**, hash size **16**, algorithm **Mean** (default) or **Gradient**
4. Enable **Geometric invariance** if you want to catch mirrored/rotated copies
5. Click **Scan**
6. Use the right preview pane to compare pairs visually before deciding
7. Manually select which image in each group to keep, delete the rest

### Pre-scanning video thumbnails via CLI for faster GUI loading

```shell
czkawka_cli video -d /your/videos --generate-thumbnails
```

This populates the thumbnail cache so Krokiet can display previews without re-computing them.

---

## Config and Cache Files

For default paths, cache file descriptions, env var overrides, and portable-drive setup see [Config/Cache files in the main guide](Instruction.md#configcache-files).

### Krokiet config files

| OS | Path |
|----|------|
| Linux | `~/.config/krokiet/` |
| macOS | `~/Library/Application Support/pl.Qarmin.Krokiet/` |
| Windows | `C:\Users\<user>\AppData\Roaming\Qarmin\Krokiet\config\` |

Note: there is no Krokiet Flatpak. The Flathub package is GTK-only and frozen at v10.0.

Key files:
- `config_general.json` - window size, theme, preset count, active preset index
- `config_preset_N.json` - all scan settings for preset N (paths, filters, tool parameters)
- `config_custom_select_state.json` - saved custom selection rules

Cache is shared with GTK and CLI (`~/.cache/czkawka/` on Linux).

---

## Tips and Tricks

For cross-frontend tips (cache management, prehash, partial scans, native CPU build) see [Tips, Tricks and Known Bugs in the main guide](Instruction.md#tips-tricks-and-known-bugs).

- **Selection visibility** - if not all columns are visible (e.g. modification date, size), use the horizontal scrollbar below the results list or narrow other columns by dragging their dividers.
