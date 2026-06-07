# Krokiet - Instructions

> **Migration notice**: Czkawka GTK 12.0 is the last version of the old GTK frontend. All users are encouraged to switch to Krokiet.

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

| Term | Definition |
|------|------------|
| **Reference path** | A directory or file added to the scan but protected from deletion or modification. Files inside appear in results for comparison only - no action is ever taken on them automatically. |
| **Included path** | A directory that will be scanned for matching files. |
| **Excluded path** | A directory that is skipped entirely during scanning. Faster than excluded items for directory-level exclusions. |
| **Excluded item** | A glob pattern filtered out from results (e.g. `*/tmp*`, `*/.git`). More flexible than excluded paths but slightly slower. |
| **Perceptual hash** | A hash computed from the visual or audio content of a file, designed so similar content produces similar hashes. Used for similar images, videos, and music content mode. |
| **Prehash** | A fast partial hash of the beginning and end of a file. Used to quickly rule out non-duplicates before computing the full hash, speeding up large scans. |
| **Hash group** | A set of files sharing the same hash (or visually similar within the chosen threshold) - these are candidate duplicates. |
| **Cache** | Computed data (hashes, thumbnails, analysis results) saved to disk so subsequent scans avoid recomputing them. Shared across all frontends (CLI, Krokiet, GTK). |
| **Preset** | A saved configuration profile storing scan directories, filters, and tool parameters. Multiple presets can be created and switched between. |
| **Similarity threshold** | For similar images/videos/music: the maximum allowed difference between two items for them to be considered similar. Lower = stricter matching. |
| **Backend / renderer** | The graphics API used to draw the Krokiet window. Options: femtovg (OpenGL, default), skia, software. Use `software` on unusual setups or when OpenGL is unavailable. |
| **Hard link** | A filesystem feature where two filenames point to the same data. By default Krokiet detects hard links and counts them only once to avoid false duplicates. |

---

## Installation

[[[Image: GitHub releases page for czkawka - show the "Assets" section of the latest release with the binary filenames highlighted (krokiet_linux_x86_64, krokiet_windows.zip, krokiet_macos) and a callout pointing to the recommended variant for each OS]]]

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

[[[Image: Full Krokiet main window annotated with six numbered callouts: (1) left side panel listing all tools, (2) top bar with Scan/Stop buttons and progress bar, (3) included/excluded directories panel below top bar, (4) main results list in the center, (5) bottom action panel with Select/Delete/Move buttons, (6) right image preview pane - draw arrows and number bubbles on each area]]]

The main window has six areas:

| # | Area | Purpose |
|---|------|---------|
| 1 | **Left panel** | Tool selector - click a tool to switch active tool and view its results |
| 2 | **Top bar** | Scan / Stop buttons, progress indicator, Settings gear icon |
| 3 | **Directory panel** | Add/remove included and excluded directories; set extension and size filters |
| 4 | **Results area** | Scan results displayed as grouped rows |
| 5 | **Bottom panel** | Selection helpers and action buttons (delete, move, rename, copy) |
| 6 | **Right pane** | Image preview for Similar Images; file metadata display |

### Left panel - tool list

[[[Image: Krokiet left side panel in isolation showing all tool entries listed vertically: Duplicate Files, Empty Folders, Big Files, Empty Files, Temporary Files, Similar Images, Similar Videos, Same Music, Invalid Symlinks, Broken Files, Bad Extensions, Bad Names, Exif Remover, Video Optimizer - with one item highlighted as active]]]

Available tools: Duplicate Files, Empty Folders, Big Files, Empty Files, Temporary Files, Similar Images, Similar Videos, Same Music, Invalid Symlinks, Broken Files, Bad Extensions, Bad Names, Exif Remover, Video Optimizer.

### Directory panel

[[[Image: Krokiet directory panel showing: the included paths list with one regular path and one path marked as "reference" (shown with a different icon or label), the excluded paths list with one entry, the extensions field containing "jpg,png", and the min size / max size fields]]]

- **Included paths** - directories that will be scanned
- **Excluded paths** - directories that will be ignored
- **Extensions** - leave empty to scan all types, or enter comma-separated values (e.g. `jpg,png`) or macros (`IMAGE`, `VIDEO`, `MUSIC`, `TEXT`)
- **Min / Max size** - filter out files outside this byte range
- **Reference path** - right-click an included path and select "Mark as reference" to protect it from deletion

### Results area

[[[Image: Krokiet results area showing a duplicate group - one bold header row reading "3 files x 1.4 MB/file = 4.2 MB total" with a group indicator icon, followed by three indented file rows each showing filename, path, size, and modification date - with one row checked/selected showing a checkbox]]]

- **Header rows** (bold) represent groups (e.g., a duplicate group or a set of similar images).
- **File rows** show name, path, size, and tool-specific extra info (e.g., image dimensions, similarity score).
- Click a file row to select/deselect it.
- Double left-click to open the file in the default application.
- Double right-click to open the containing folder.
- Right-click for context menu (open, copy path, compare images, etc.).

### Bottom action panel

[[[Image: Krokiet bottom panel showing all visible buttons: "Select all", "Deselect all", "Select all except oldest", "Select all except newest", "Select all except biggest", "Select all except smallest", "Invert selection", "Custom selection", "Delete", "Move to trash", "Move", "Copy" - with the "Custom selection" button highlighted and its popup partially visible]]]

**Selection buttons:**
- Select all / Deselect all
- Select all except oldest / newest / biggest / smallest - useful starting points for deduplication
- Invert selection - flip checked/unchecked state for all items
- Custom selection - open a popup with advanced rules (path patterns, extension filters, etc.)

**Action buttons:**
- **Delete** - permanently delete selected files
- **Move to trash** - move to system trash (recoverable)
- **Move** - move to a chosen directory
- **Copy** - copy to a chosen directory
- **Rename** (Bad Extensions / Bad Names only) - apply suggested fix to file name

---

## Tool-by-Tool Guide

### Duplicate Files

Finds files with identical content (or matching name/size, depending on the method chosen).

[[[Image: Krokiet tool settings panel for Duplicate Files showing: check method dropdown (Hash selected), hash type dropdown (BLAKE3 selected), minimum file size input field, prehash cache toggle switch]]]

**Check methods** (fastest to most reliable):

| Method | Speed | Reliability | Notes |
|--------|-------|-------------|-------|
| Name | Fastest | Low | Compares filenames only; many false positives |
| Size | Fast | Low | Compares file sizes only; almost always has false positives |
| Size + Name | Fast | Low | Slightly better than size alone |
| Hash | Slow | ~100% | Recommended for real deduplication |

**Hash types** (used with Hash method):

| Hash | Speed | Recommended |
|------|-------|-------------|
| BLAKE3 | Fast | Yes - cryptographic, default |
| XXH3 | Fastest | When speed matters most |
| CRC32 | Medium | Legacy use cases |

**Prehash cache** - speeds up re-scans by caching partial hashes (first and last 4 KB) of large files. Enabled by default.

### Empty Folders

Finds directories that contain no files or subdirectories. Uses a recursive algorithm that propagates "not empty" status upward through the directory tree.

[[[Image: Krokiet Empty Folders results showing a flat list of folder paths with columns for path and size (all showing 0 bytes), no groups/headers needed]]]

### Big Files

Finds the N largest (or N smallest) files in the specified directories. Configure the count (default 50) and mode (largest/smallest) in the tool settings.

### Empty Files

Finds files with 0 bytes. Optionally also detects:
- Files filled entirely with null bytes (`\0`)
- Files filled entirely with non-printable ASCII characters (implies null bytes)

### Temporary Files

Finds files matching common temporary file patterns. Default patterns: `.tmp`, `.bak`, `.part`, `.crdownload`, `.temp`, `.cache`, `.dmp`, `.download`, `.partial`, `thumbs.db`, `~`, `#`.

The extension list can be customized in settings.

### Similar Images

Finds images that look alike but are not byte-for-byte identical (different resolution, watermarks, compression artifacts, JPEG re-save, etc.).

[[[Image: Krokiet Similar Images settings panel showing: similarity slider or number input set to 10, hash algorithm dropdown showing "Mean", hash size buttons with "16" selected, resize filter dropdown showing "Lanczos3", geometric invariance toggle set to off, and the exclude same size/resolution toggles]]]

**Key settings:**

| Setting | Default (Krokiet) | Notes |
|---------|-------------------|-------|
| Similarity | 10 | Max hash distance (0-40). Raise if too few matches, lower for stricter results. |
| Hash algorithm | Mean | Mean is default; Gradient is also good for photos; Blockhash is unique - does not resize before hashing |
| Hash size | 16 | Higher = more precise comparison; requires higher similarity threshold |
| Resize filter | Lanczos3 | Lanczos3 is highest quality (default); Nearest is fastest |
| Geometric invariance | off | Enable to also find mirrored or rotated copies of images |
| Exclude same size | off | Skip groups where all images have identical byte size |
| Exclude same resolution | off | Skip groups where all images have identical pixel dimensions |

[[[Image: Krokiet Similar Images results with the right pane active - left side shows two groups each with a header and image file rows, the right preview pane shows a thumbnail of the currently selected image with its resolution and path below it]]]

### Similar Videos

Finds visually similar videos using perceptual frame hashing. Requires **ffmpeg** installed.

**Key settings:**
- **Tolerance** (0-20) - maximum frame difference; Krokiet default is 15
- **Duration tolerance %** - how much video lengths may differ and still be compared
- **Window count** - temporal samples per video (more = more accurate, slower)
- **Min matching windows** - fraction of windows that must match to call two videos similar
- **Audio comparison** - also compare by audio fingerprint; very resource-intensive, opt-in

[[[Image: Krokiet Similar Videos results showing two video groups - header row with "2 similar videos", file rows showing filename, path, size, and duration for each video - with a thumbnail image visible in the right preview pane]]]

### Same Music

Finds duplicate or similar music files.

**Methods:**
- **Tags** - compare by metadata (artist, title, album, year, bitrate, genre, length); fastest
- **Content** - compare by audio fingerprint; more accurate, much slower

### Invalid Symlinks

Finds symbolic links whose target does not exist or that form a circular chain (exceeds 20 jumps).

### Broken Files

Finds files that fail to open with their expected library (corrupted or truncated content).

**Supported types**: Images, Audio, PDF, Archives (zip, 7z, gz, tar, zst, bz2, xz), Fonts (ttf, otf, ttc), Markup (JSON, XML, TOML, YAML, SVG) - all enabled by default. Video (fast ffprobe check or full ffmpeg decode) - disabled by default, requires ffmpeg.

Note: false positives can occur depending on the library used. Always verify before deleting.

### Bad Extensions

Finds files whose content (detected from magic bytes at the start of the file) does not match their extension.

[[[Image: Krokiet Bad Extensions results - table with columns: filename, path, current extension, "Proper Extension" column showing "(detected_type) compatible_ext_list" e.g. "(7z) rar zip p7" - with one row selected and the Rename button visible in the bottom panel]]]

The "Proper Extension" column shows `(detected type)` and all compatible extensions. Use the **Rename** button to apply the suggested fix.

### Bad Names

Finds files with problematic names. Each check can be enabled independently:
- **Uppercase extensions** (e.g. `.JPG`) - enabled by default
- **Emoji in filename** - enabled by default
- **Spaces at start or end of name** - enabled by default
- **Non-ASCII graphical characters** - enabled by default
- **Characters outside a configured restricted charset** - disabled by default (requires defining the allowed charset)
- **Duplicated non-alphanumeric characters** (e.g. `file__name`, `doc---final`) - disabled by default

Results include a suggested corrected name in the "Proper Name" column.

### Exif Remover

Finds image files containing EXIF metadata (GPS coordinates, camera model, creation date, etc.). Can remove all tags or skip a user-specified list.

Supported formats: `jpg`, `jpeg`, `jfif`, `png`, `tiff`, `tif`, `avif`, `jxl`, `webp`, `heic`, `heif`.

### Video Optimizer

Identifies videos suitable for optimization. Two modes:

**Transcode** - find videos using inefficient codecs (e.g., old H.264 files that could be converted to H.265/AV1 to save space).

**Crop** - find videos with black bars (letterbox/pillarbox) that can be cropped.

[[[Image: Krokiet Video Optimizer results in Transcode mode - list of video files showing filename, path, size, and current codec column - with codec column highlighting entries that are not in the excluded codec list]]]

---

## Settings

[[[Image: Krokiet settings screen open on the General tab - language dropdown showing current language, theme toggle showing Light/Dark, UI scale factor input, audio notifications toggle, and tabs for General / Performance visible at top]]]

Open settings via the gear icon in the top bar. Organized into tabs:

### General
- **Language** - UI language (takes effect after restart)
- **Theme** - Light or Dark
- **UI scale** - display scaling factor (useful on HiDPI screens)
- **Notifications** - system/audio notification when scan completes

### Performance
- **Thread count** - CPU threads to use (0 = all available)
- **Use cache** - enable/disable hash and thumbnail cache
- **Prehash cache** - cache partial hashes of large files to speed up re-scans (enabled by default)
- **Delete outdated cache entries automatically** - auto-clean stale entries on each scan (recommended on)

### Presets

[[[Image: Krokiet preset area - a dropdown showing preset names like "Preset 1", "Home scan", "Work scan", with Load/Save/Delete buttons next to the dropdown]]]

Each preset stores: included/excluded directories, extension and size filters, tool-specific parameters.

Presets are saved at `~/.config/krokiet/config_preset_N.json` (Linux).

---

## Common Workflows

### Removing duplicates while protecting a backup

1. Add your working folder to **Included paths**
2. Add your backup folder to **Included paths**
3. Right-click the backup folder in the list and choose **Mark as reference path**
4. Select **Duplicate Files** tool, set method **Hash**, hash type **BLAKE3**
5. Click **Scan**
6. Use **Select all except newest** (or whichever strategy fits)
7. Verify that no reference-path files are selected
8. Click **Move to trash** or **Delete**

### Cleaning up a photo library (similar images)

1. Add all photo directories to **Included paths**
2. Select **Similar Images** tool
3. Set similarity **10**, hash size **16**, algorithm **Mean** (default) or **Gradient**
4. Enable **Geometric invariance** if you want to catch mirrored/rotated copies
5. Click **Scan**
6. Use the right preview pane to compare pairs visually before deciding
7. Manually select which image in each group to keep, delete the rest

### Scanning on a portable drive (portable mode)

Keep config and cache on the drive alongside the binary:

```shell
CZKAWKA_CONFIG_PATH="$(dirname "$(realpath "$0")")/config"
CZKAWKA_CACHE_PATH="$(dirname "$(realpath "$0")")/cache"
./krokiet
```

Save this as `open_krokiet.sh` on the drive next to the binary and make it executable.

### Pre-scanning video thumbnails via CLI for faster GUI loading

```shell
czkawka_cli video -d /your/videos --generate-thumbnails -N -M -W
```

This populates the thumbnail cache so Krokiet can display previews without re-computing them.

---

## Config and Cache Files

### Config files (per frontend - not shared between Krokiet and GTK)

| OS | Path |
|----|------|
| Linux | `~/.config/krokiet/` |
| Linux Flatpak | `~/.var/app/com.github.qarmin.czkawka/config/krokiet/` |
| macOS | `~/Library/Application Support/pl.Qarmin.Krokiet/` |
| Windows | `C:\Users\<user>\AppData\Roaming\Qarmin\Krokiet\config\` |

Key files:
- `config_general.json` - window size, theme, preset count, active preset index
- `config_preset_N.json` - all scan settings for preset N (paths, filters, tool parameters)
- `config_custom_select_state.json` - saved custom selection rules

Override location:
```shell
CZKAWKA_CONFIG_PATH="/custom/path/config" krokiet
```

### Cache files (shared across Krokiet, GTK, and CLI)

| OS | Path |
|----|------|
| Linux | `~/.cache/czkawka/` |
| Linux Flatpak | `~/.var/app/com.github.qarmin.czkawka/cache/czkawka/` |
| macOS | `~/Library/Caches/pl.Qarmin.Czkawka/` |
| Windows | `C:\Users\<user>\AppData\Local\Qarmin\Czkawka\cache\` |

Notable files:
- `cache_duplicates_<HASH>.txt` - duplicate file hashes
- `cache_similar_image_<SIZE>_<HASH>_<FILTER>.bin/.json` - perceptual image hashes
- `cache_similar_videos.bin/.json` - video signatures
- `cache_broken_files.txt` - broken file check results

Files with `.json` extension can be edited manually (useful when moving files between disks). The `.bin` file is loaded by default; if missing, the `.json` fallback is used.

Override cache location:
```shell
CZKAWKA_CACHE_PATH="/custom/path/cache" krokiet
```

---

## Tips and Tricks

- **Slow cache loading** - delete the relevant `cache_similar_image_*.bin` file; it regenerates on next scan with only the currently scanned files in it.

- **Partial scans** - you can stop a scan mid-way; computed hashes are already saved to cache and will speed up the next full scan.

- **Faster hashing with native CPU** - compile with native CPU instructions for a 10-20% boost on image/duplicate hashing:
  ```shell
  RUSTFLAGS="-C target-cpu=native" cargo build --release
  ```

- **Prehash cache** - enabled by default in Krokiet. Caches partial hashes (first and last 4 KB) of large files so re-scans only need to fully hash new or changed files. Disable only if the cache file size is a concern.

- **Persistent cache for removable drives** - disable "Delete outdated cache entries automatically" when scanning external drives you regularly unplug. Use "Remove outdated results" button manually instead to avoid entries being evicted on unplug.

- **Right-click context menu** - right-click any result row to open the file, open its folder, copy the path, or (for similar images) launch the image comparison tool.

- **Image comparison tool** - in Similar Images results, select two images and open the comparison popup. It shows a side-by-side diff with differences highlighted, so you can make an informed decision about which copy to keep.

- **Selection visibility** - if not all columns are visible (modification date, size), use the horizontal scrollbar below the results list or narrow other columns by dragging their dividers.
