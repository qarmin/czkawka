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
| **Backend / renderer** | The graphics API used to draw the Krokiet window. Options: femtovg (OpenGL, default), femtovg-wgpu, vello, software. Use `software` on unusual setups or when OpenGL is unavailable. |
| **Hard link** | A filesystem feature where two filenames point to the same inode. Krokiet detects hard links and counts them only once by default to avoid false duplicates. |

---

## Installation

Prebuilt binaries (no extra dependencies required) are available for Windows 10/11, macOS, and Linux (glibc 2.17+, so almost every distribution released after 2014).

Download from the [GitHub releases page](https://github.com/qarmin/czkawka/releases).

For the **Similar Videos** tool, **ffmpeg** must be installed separately:

| OS | Command |
|----|---------|
| Linux | `sudo apt install ffmpeg` |
| macOS | `brew install ffmpeg` |
| Windows | `choco install ffmpeg` or download from ffmpeg.org and place `ffmpeg.exe` in `PATH` |

### Compile from source

```shell
# Simplest - installs the latest optimized binary
cargo install krokiet --locked
```

### Alternative renderers

By default femtovg (OpenGL) and software renderers are built in. Switch at runtime:

```shell
SLINT_BACKEND=software ./krokiet        # pure software rendering, safest fallback
SLINT_BACKEND=winit-femtovg ./krokiet  # OpenGL via femtovg (default)
SLINT_BACKEND=femtovg-wgpu ./krokiet   # femtovg on wgpu (requires femtovg_wgpu compile feature)
SLINT_BACKEND=vello ./krokiet          # Vello on wgpu (requires vello compile feature)
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
| 1 | **Left panel** | Tool selector - logo at top (opens About; the `?` icon next to it opens the built-in Help), 15 tools listed vertically, Settings gear and the tool options button at the bottom. The arrow in the top left corner collapses the panel |
| 2 | **Results area** | Scan results displayed as grouped rows; occupies the bulk of window height |
| 3 | **Action bar** | Below the results area - Scan/Stop buttons plus selection helpers and action buttons (delete, move, rename, hardlink, ...). Right side has two toggle buttons: folder icon (show/hide directory panel) and info icon (show/hide error log). |
| 4 | **Status bar** | Immediately below the action bar - read-only text line showing current scan state or result summary ("Found N items in Xms"). |
| 5 | **Directory panel** | Togglable panel below the status bar (folder icon button). Shows Included Paths (left) and Excluded Paths (right). Extensions, size filters, and excluded items are in Settings, not here. |
| 6 | **Preview panel** | Optional preview of the highlighted row on the right (Similar Images and Duplicate Files when image preview is enabled, Similar Videos and Video Optimizer when video thumbnails are enabled). Hidden when several rows are highlighted. The splitter can be dragged. Tool options are not here, they open in a popup from the options button in the left panel. |

### Left panel - tool list

Available tools: Duplicate Files, Empty Folders, Big Files, Empty Files, Temporary Files, Similar Images, Similar Videos, Music Duplicates, Invalid Symlinks, Broken Files, Bad Extensions, Bad Names, Exif Remover, Video Optimizer, Disk Usage.

The gear opens Settings (hidden while Settings is open). The button with cogs next to it opens the options of the selected tool in a popup and is shown only for tools that have options. Clicking the logo opens About.

### Directory panel

Toggled by the folder icon button on the right side of the action bar. The info button next to it shows the error log in the same place; clicking the active button hides the panel. Contains only:

- **Included paths** - directories that will be scanned. Each row has a "Ref" checkbox to mark it as a reference path (protected from deletion) and a button that removes it. Double click on a path opens it, right click opens its parent folder. Add paths with the folder/file/manual-entry buttons at the top. If every included path is a reference path the scan does not start.
- **Excluded paths** - directories that will be ignored during scanning. Add with the same buttons.

Extensions, size filters, excluded items, and recursive search are configured in **Settings**, not in this panel.

### Results area

- **Header rows** (bold) represent groups (e.g., a duplicate group or a set of similar images).
- **File rows** show name, path, size, and tool-specific extra info (e.g., image dimensions, similarity score).
- Every row has two separate states: **highlighted** (the row you point at) and **checked** (the checkbox in the first column). Actions such as Delete or Move work on the checked rows.
- Click highlights a row and shows its preview. Ctrl+click adds or removes a single row, Shift+click highlights a range. Clicking never checks a row; use the checkbox, the Space key or the Select popup.
- Double left-click opens the file in the default application.
- Right-click highlights the row and opens a context menu (not on group headers): Open Item, Open Parent Folder, Remove from Results, Remove All from Folder, Remove All from Folder (recursive), Select All from Folder, Select All from Folder (recursive), Exclude Parent Folder, Exclude Item, Rename, Copy File Name, Copy Parent Folder, Copy Full Path. The remove entries only hide rows; files stay on the disk. Select All from Folder checks rows and, in group tools, leaves one item of each group unchecked. The Exclude entries add the path to Excluded Paths and remove matching rows. Rows from reference folders are protected from remove, exclude and select entries.
- Click a column header to sort by it (first click descending, then the direction toggles); the header of the checkbox column sorts by checked state. The Sort button offers Sort by full name, Sort by focus (highlighted rows go to the top) and Reverse order.
- Keyboard (after clicking a row): Up/Down, PageUp/PageDown, Home/End move the highlight (Shift extends the range, headers are skipped), Space checks or unchecks all highlighted rows, Enter opens the current item, Ctrl+A highlights all rows (it does not check them).
- Drag the dividers of the column headers to resize columns.

### Action bar

The action bar sits below the results area. Buttons visible depend on the active tool and whether results exist.

**Always present (when not scanning):**
- **Scan** - start the scan for the active tool
- **Select** - open a popup with selection modes (needs results). Select all, Unselect all, Invert selection and Custom are in every tool. Duplicate Files, Similar Images, Similar Videos and Music Duplicates also have: One oldest/newest/biggest/smallest, All except oldest/newest/biggest/smallest (Similar Images also by resolution), shortest/longest path and Invert selection in group; each can be hidden in the settings. The modes that pick one file or all except one leave one file of each group unchecked; Select all and the invert modes do not, so the delete popup reports how many groups have every file checked
- **Move** - move checked files to a chosen directory; the popup can copy instead of move, preserve the folder structure and rename on conflict instead of skipping (needs checked rows)
- **Delete** - permanently delete checked files (needs checked rows)
- **Trash** - move checked files to system trash (recoverable; may fail on network shares) (needs checked rows)
- **Save** - ask for a folder and write the results there as three files: `<name>.txt`, `<name>_pretty.json` and `<name>_compact.json` (needs results). Results cannot be loaded back
- **Sort** - open sort options popup (needs results)

**Tool-specific buttons (only shown for relevant tool):**
- **Rename** - apply suggested fix to file name (Bad Extensions and Bad Names)
- **Optimize** - transcode / crop selected videos (Video Optimizer)
- **Clean** - strip EXIF metadata from selected files (Exif Remover)
- **Compare** - open side-by-side image comparison overlay (Similar Images, when results are present)
- **Hardlink** / **Softlink** - Duplicate Files, Similar Images, Similar Videos and Music Duplicates only. In a group with a reference file the links point to it; otherwise the first checked file is kept and the other checked files of the group become links to it, so check at least two files per group

---

## Tool-by-Tool Guide

### Duplicate Files

Finds files with identical content (or matching name/size, depending on the method chosen). With the Hash method files are grouped by size, then by a prehash (first and last 4 KB; smaller files are hashed whole) and finally by a hash of the whole file - no byte-by-byte comparison. The size/name methods do not read files. Hard links are hidden by default (global setting) and Hardlink/Softlink actions are available.

**Sub-settings (options popup):**

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
- Image preview - show preview of the highlighted duplicate
- Minimal size of cached files - Hash - smaller files are not stored in the hash cache (default 256; known issue: the UI labels it KB but the core compares it with the size in bytes)
- Use prehash - whether the partial hashes are saved in the cache (the prehash stage itself always runs)
- Minimal size of cached files - Prehash (default 256, same unit issue)

### Empty Folders

Finds directories that contain no files. A folder holding only empty folders is empty too and only the topmost folder of such a branch is listed (deleting it removes the whole branch). Any other entry (including a symlink), an excluded subfolder, an unreadable folder or one on another filesystem makes a folder non-empty. Size and extension filters do not apply. No sub-settings.

### Big Files

Finds the N largest (or N smallest) files in the specified directories. The global minimal/maximal size filters are ignored and 0-byte files are never listed.

**Sub-settings (options popup):**
- Method: The Biggest / The Smallest
- Number of files (default 50)

### Empty Files

Finds files with 0 bytes.

**Sub-settings (options popup) - additional file types to find:**
- **Files filled with null bytes** - also find non-empty files whose entire content is null bytes (0x00); disabled by default
- **Files filled with non-printable characters** - also find files in which every byte is not a visible ASCII character (null, space, tab, new lines and also all bytes above 127, so some non-Latin text files match); disabled by default

The global size filters are ignored by this tool.

### Temporary Files

Finds files whose lowercased name ends with one of the listed suffixes (case-insensitive). The list must not be empty; size filters do not apply.

**Sub-settings (options popup):**
Editable extension/pattern list, with a Reset button. Default list:
`#,thumbs.db,.bak,~,.tmp,.temp,.ds_store,.crdownload,.part,.cache,.dmp,.download,.partial`

### Similar Images

Finds images that look alike but are not byte-for-byte identical (different resolution, watermarks, compression artifacts, JPEG re-save, etc.).

**Sub-settings (options popup):**

| Setting | Default | Notes |
|---------|---------|-------|
| Hash size | 16 | Options: 8, 16, 32, 64. Higher = more precise; requires higher max difference threshold. |
| Resize algorithm | Lanczos3 | Options: Lanczos3, Nearest, Triangle, Gaussian, CatmullRom. |
| Hash type | Mean | Options: Mean, Gradient, BlockHash, VertGradient, DoubleGradient, Median. BlockHash does not resize before hashing. |
| Geometric invariance | Off | Options: Off, Mirror + Flip, Mirror + Flip + Rotate 90. |
| Ignore images with same size | off | Skip groups where all images have identical byte size. |
| Ignore images with same resolution | off | Skip groups where all images have identical pixel dimensions. |
| Max difference | 10 | Max hash distance, range 0-40 for every hash size (hash size 8 has only 64 bits, so 40 matches almost everything). Raise for more matches, lower for stricter. |

Hash size, hash type, resize algorithm and geometric invariance are part of the cache key; changing them re-hashes the images. EXIF orientation is applied before hashing. Supported extensions: jpg/jpeg, png, tiff/tif, tga, ff, jif/jfi, bmp, webp, exr, qoi, jxl and RAW formats (built-in `rawler` decoder); HEIF/HEIC needs the `heif` feature and AVIF the `libavif` feature; the optional `libraw` feature only replaces the RAW decoder. GIF and ICO are not scanned.

**In global Settings (under "Similar Images tool" section):**
- Image preview - show preview of the highlighted image

### Similar Videos

Finds visually or acoustically similar videos. Visual mode samples frames (engine: `similario_core`), compares sequences in short windows and can also find a shorter clip inside a longer video (subclip). Visual mode and thumbnails require **ffmpeg** and **ffprobe** in `PATH`; the audio fingerprint mode does not.

**Sub-settings (options popup):**

Toggle at the top: **Compare by audio fingerprint** - switches between visual and audio comparison mode.

**Visual mode** (default):
- Quick preset: Custom / Near-identical / Similar / Movies (long skip). Non-custom presets set the sliders below automatically.
- When Custom (sliders are shown only then): Max difference (0-20, default 15), Skip duration [s] (0-300, default 15, capped at 15% of the video length), Video hash duration - length of every sampled window (2-60 s, default 10), Window count (1-20, default 5), Duration tolerance [%] (0-100, default 20), Min matching windows (20-100%, default 60%), Subclip min match (20-100%, default 50%)
- Letterbox crop detect (Custom only, on by default)
- Preset values (max difference / windows / skip / hash duration / duration tolerance / min windows / subclip): Near-identical 3 / 8 / 15 / 10 / 10% / 80% / 70%; Similar 14 / 5 / 15 / 10 / 25% / 50% / 40%; Movies (long skip) 8 / 8 / 90 / 12 / 15% / 60% / 50%
- The visual cache key contains skip, hash duration, crop detection and window count; grouping parameters do not invalidate it
- Ignore videos with same size
- Ignore videos with same resolution

**Audio fingerprint mode:**
- Quick preset: Custom / Identical videos / Clip in longer video / Similar content
- When Custom: Similarity [%] (0-100, default 80), Min length ratio (shorter/longer, 0-1, default 0.1), Min file duration [s] (0-600, default 10; shorter files are dropped), Max audio difference (0-10, default 3)
- Ignore videos with same size
- Ignore videos with same resolution

**In global Settings (under "Similar Videos tool" and "Video Thumbnails" sections):**
- Generate thumbnails - enable thumbnail generation for results preview
- Thumbnail position in video (%) (default 10)
- Generate thumbnail grid instead of single image
- Number of tiles per side in thumbnail grid
- Delete unused video thumbnails older than 7 days at app startup

### Music Duplicates

Finds duplicate or similar music files.

**Sub-settings (options popup):**

Audio check type: **Tags** (default) or **Fingerprint**.

When Tags:
- Approximate Tag Comparison toggle - applies only to Title and Artist (ignores case, bracketed text and punctuation)
- Compared tags (individually toggleable): Title (on), Artist (on), Bitrate (off), Genre (off), Year (off), Length (off). All selected tags must match; files with an empty tag are not grouped by it

When Fingerprint:
- Compare within groups of similar titles toggle (off by default; files with an empty title are skipped)
- Max difference slider (0-10, default 3)
- Minimal fragment duration slider (0-180 s, default 5)

### Invalid Symlinks

Finds symbolic links whose target does not exist or that form a circular chain. A link in the middle of a chain that is not itself a link is skipped. Deleting removes only the link. No sub-settings.

### Broken Files

Finds files that fail to open with their expected library (corrupted or truncated content).

**Sub-settings - file types to check (defaults shown; files are selected by extension):**
- Audio - on by default
- PDF - on by default
- Archive (zip, jar, 7z, gz, tar, zst, bz2, xz) - on by default
- Image - on by default
- Video (ffprobe) - off; quick header check, requires ffmpeg
- Video (ffmpeg) - off; full decode, very slow, requires ffmpeg
- Font - on by default
- Markup (JSON/XML/TOML/YAML/SVG) - on by default

Videos are not checked at all unless one of the two video options is enabled.

Note: false positives can occur depending on the library used. Always verify before deleting.

### Bad Extensions

Finds files whose content (detected from magic bytes with the `infer` crate) does not match their extension. No sub-settings; some extensions are never reported. Rename is skipped with a warning when the target name already exists.

The "Proper Extension" column shows the detected type and compatible extensions. Use the **Rename** button to apply the suggested fix.

### Bad Names

Finds files with problematic names.

**Sub-settings (options popup) - checks (defaults shown):**
- Uppercase extension (e.g. `.JPG`) - on by default
- Emoji in name - on by default
- Leading/trailing spaces - on by default
- Non-ASCII chars (e.g. ą, ć, ñ) - on by default; suggests ASCII equivalents
- Limited charset - off; transliterates to ASCII then removes chars outside 0-9a-zA-Z plus user-defined allowed chars (in the name and the extension); when enabled, an "Allowed chars" text field appears (default: space and `_ . , - ( ) [ ] ! ' "`)
- Duplicated chars (e.g. `file---name`) - off; merges consecutive duplicated non-alphanumeric characters

The non-ASCII check also drops the glob characters `* ? [ ]`; a name that becomes empty is replaced by `empty`. Rename fails with a warning if the target name exists.

Results include a suggested corrected name.

### Exif Remover

Finds image files containing EXIF metadata (GPS coordinates, camera model, creation date, etc.).

**Sub-settings (options popup):**
- Ignored tags - comma-separated list of exact, case-sensitive EXIF tag names to preserve (default `Orientation`; leave empty to strip all tags)

Use the **Clean** button to remove EXIF from checked files. The popup has an **Overwrite files** option (off by default): without it the original stays untouched and a copy `name.czkawka_cleaned_exif.<ext>` is written. Only formats supported by the cleaning library are handled (JPEG, PNG, TIFF, WebP, JXL, HEIF). Results are cached.

### Video Optimizer

Identifies videos suitable for optimization.

**Sub-settings (options popup):**

Mode: **Transcode** (default) or **Crop**.

When Crop:
- Crop type: Black Bars (default) / Static Content (has its own image threshold)
- Black pixel threshold (default 20, range 0-128)
- Black bar min percentage (default 90, range 50-100)
- Max samples (default 60), minimal crop size (default 20)

When Transcode:
- Excluded codecs - codecs to skip (default: `h265,hevc,av1,vp9`; Reset button available)

Use the **Optimize** button to process checked videos. For transcoding the popup offers the codec (default h265), a hardware encoder (Nvenc, Vaapi, Qsv, VideoToolbox, Amf), CRF (default 23), noise reduction, fail-if-bigger, overwrite, size limit (1920x1920) and fps limit (60) and a custom ffmpeg command. Requires ffmpeg.

### Disk Usage

Shows what takes space on disk as a Treemap (nested rectangles) or Rings (sunburst) chart. It uses the Included and Excluded Paths from the bottom panel and the options button in the left panel.

- Left click selects an item, right click selects it and opens a menu (Open file/folder, Open parent folder, Copy path, Move to trash), double click opens the item with the system application. Hovering shows a card with name, size, share and type. Scroll up zooms one level into the selected folder, scroll down (or the Zoom out button) goes out one level.
- Nine file type toggles above the chart hide or show a type; the borders icon toggles nested borders. In Rings mode the Levels slider (2-8, default 6) sets the number of rings and a folder list is shown on the left.
- Options (popup, not saved in presets, reset on restart): Ignore excluded items (on by default: the excluded items patterns are not applied), Ignore excluded folders (off by default), File type detection (Extension only / Content of large files - default, files of at least 1 MiB / Content of all files; only for files with an unknown extension), and Detail sliders: Group entries smaller than (1/100 to 1/10000000 of the parent, default 1/1000000), Smallest entry shown (1-10 px, default 1) and Folders narrower than this are one block (0-60 px, 0 = off).
- Symlinks are not followed and have size 0, hard links are counted once per name, sizes are file sizes (not allocated space), other filesystems stay excluded.
- Known limitation: Move to trash currently only removes the item from the chart; the file is not yet moved to the trash.

---

## Settings

Open settings via the gear icon at the bottom of the left panel. Settings is a single scrollable list with section headers - not a tabbed view.

**Scan filters** (these are here, not in the directory panel):
- **Allowed extensions** - leave empty to scan all types, or enter comma-separated values (e.g. `jpg,png`). Supports keywords: `image`, `video`, `music`, `text` (expanded to the relevant extension lists).
- **Excluded extensions** - extensions to skip
- **Excluded items** - comma-separated glob patterns matched against the whole path (e.g. `*/tmp*,*/.git/*`). Every pattern must contain `*`; `DEFAULT` expands to the default list (trash, `.git`, `node_modules`, caches, system folders; on Linux also hidden folders under `/home`). Default excluded paths are pre-filled in the directory panel (`/proc,/dev,/sys,/snap` or `C:\Windows`)
- **File size (KB)** - min and max file size filter
- **Recursive search** - toggle whether subdirectories are scanned
- **Ignore other filesystems** - Linux only, off by default

**General:**
- **Language** - UI language; changes apply immediately, no restart needed
- **Dark theme** - toggle dark/light theme; applies immediately
- **Show only icons** - hide button labels in the action bar
- **Hide hard links** - count hard-linked files only once in results (on by default; applied in Duplicate Files, Similar Images and Similar Videos)
- **Load window size**, **play sound** and **show notification** when a scan ends, buttons that open the config and cache folders, and counters of cache, thumbnail and log size

**Performance:**
- **Thread number** - CPU threads to use, 0 means all; requires restart to take effect
- **Use cache** - enable/disable hash and thumbnail cache
- **Use prehash** (Duplicate Files section) - cache partial hashes of large files to speed up re-scans
- **Delete automatically outdated cache entries** - auto-clean stale cache records (verified at most once per week, timestamps in `cleaning_timestamps.json` in the cache folder). The **Clean outdated cache** button removes entries of files that no longer exist or changed
- **Application scale** - manual UI scale factor; works only when **Use manual application scale** is checked (otherwise DPI scaling is automatic); requires restart to take effect

### Presets

Each preset stores: included/excluded paths, extension and size filters, tool-specific parameters.

There are 10 user presets plus one reserved slot used for folders passed on the command line. Presets are saved at `~/.config/krokiet/config_preset_0.json` ... `config_preset_10.json` (Linux; the number is the UI number minus one). Language, theme, scale, window size and preset names are global and live in `config_general.json`. The preset Save/Load/Reset buttons act immediately; other settings are written when the application exits normally (not after a crash).

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

Key files:
- `config_general.json` - window size, theme, preset count, active preset index
- `config_preset_N.json` - all scan settings for preset N (paths, filters, tool parameters)
- `config_custom_select_state.json` - saved custom selection rules

Cache is shared with CLI (`~/.cache/czkawka/` on Linux).

---

## Tips and Tricks

For cross-frontend tips (cache management, prehash, partial scans, native CPU build) see [Tips, Tricks and Known Bugs in the main guide](Instruction.md#tips-tricks-and-known-bugs).

- **Selection visibility** - if not all columns are visible (e.g. modification date, size), use the horizontal scrollbar below the results list or narrow other columns by dragging their dividers.
