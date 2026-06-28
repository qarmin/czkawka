# Czkawka CLI - Instructions

Czkawka CLI is a terminal frontend for `czkawka_core` that exposes all scanning tools as subcommands, suitable for scripting and automation.

## Table of Contents

- [Glossary](#glossary)
- [Requirements](#requirements)
- [Quick Start](#quick-start)
- [Common Flags](#common-flags)
- [Output Formats and Exit Codes](#output-formats-and-exit-codes)
- [Tools](#tools)
  - [dup - Duplicate Files](#dup---duplicate-files)
  - [empty-folders](#empty-folders)
  - [big - Big Files](#big---big-files)
  - [empty-files](#empty-files)
  - [temp - Temporary Files](#temp---temporary-files)
  - [image - Similar Images](#image---similar-images)
  - [video - Similar Videos](#video---similar-videos)
  - [music - Same Music](#music---same-music)
  - [symlinks - Invalid Symlinks](#symlinks---invalid-symlinks)
  - [broken - Broken Files](#broken---broken-files)
  - [ext - Bad Extensions](#ext---bad-extensions)
  - [bad-names](#bad-names)
  - [exif-remover](#exif-remover)
  - [video-optimizer](#video-optimizer)
- [Automation Examples](#automation-examples)

---

## Glossary

For shared concepts (reference paths, cache, prehash, hard links) see [Terminology in the main guide](Instruction.md#terminology-shared-across-all-frontends). CLI-specific terms below.

| Term | Definition |
|------|------------|
| **Included directory** (`-d`) | Directory to scan. Required for all tools. Multiple values can be listed. |
| **Excluded directory** (`-e`) | Directory skipped entirely during scanning. Faster than excluded items for folder-level exclusions. |
| **Excluded items** (`-E`) | Glob patterns matched against full paths (e.g. `*/tmp*`, `*/.git`). More flexible than `-e` but slightly slower. |
| **Reference directory** (`-r`) | A directory whose files appear in results for comparison only. Files inside are never deleted or modified by `--delete-method`. Available for: `dup`, `image`, `video`, `music`. |
| **Delete method** (`-D`) | Selects which file in each group is kept and which are removed. Codes: AEN, AEO, AEB, AES, ON, OO, OB, OS, HARD, NONE. See the [dup section](#dup---duplicate-files) for details. |
| **Dry run** (`-Q`) | Shows what operations would be performed without executing them. Always use before a real deletion run. |
| **Extension macros** | Shorthand groups for `-x`/`-P`: `IMAGE` (jpg,kra,gif,png,bmp,tiff,...), `VIDEO` (mp4,flv,mkv,webm,...), `MUSIC` (mp3,flac,ogg,...), `TEXT` (txt,doc,docx,...). |

---

## Requirements

Prebuilt binaries work on Linux (Ubuntu 22.04+), Windows 10+, and macOS 10.15+.

Optional runtime dependencies:
- **ffmpeg** - required for `video` (similar videos) and `video-optimizer`
- **libheif / libraw / libavif** - only if compiled from source with those features

Install ffmpeg:

| OS | Command |
|----|---------|
| Linux | `sudo apt install ffmpeg` |
| macOS | `brew install ffmpeg` |
| Windows | `choco install ffmpeg` or download from ffmpeg.org and place `ffmpeg.exe` in `PATH` |

Compile from source:
```shell
cargo build --release --bin czkawka_cli
# With optional image format support:
cargo build --release --bin czkawka_cli --features "heif,libraw,libavif"
```

---

## Quick Start

<img width="600" alt="Screenshot From 2026-06-28 07-19-24" src="https://github.com/user-attachments/assets/4a98567a-68b1-4be9-a6de-e7eb5f9d9ca3" />


```shell
# List all tools
czkawka_cli --help

# Show options for a specific tool
czkawka_cli dup --help

# Find duplicate files in a directory (results printed to console)
czkawka_cli dup -d /home/user/Documents

# Save results to file without printing
czkawka_cli dup -d /home/user/Documents -f results.txt -N

# Dry run - preview which files would be deleted (keeps newest in each group)
czkawka_cli dup -d /home/user/Documents -D AEN -Q

# Delete duplicates keeping newest, move others to trash
czkawka_cli dup -d /home/user/Documents -D AEN -y
```

---

## Common Flags

These flags are available for every tool:

| Flag | Long | Default | Description |
|------|------|---------|-------------|
| `-d` | `--directories` | (required) | One or more directories to scan |
| `-e` | `--excluded-directories` | - | Directories to skip entirely |
| `-E` | `--excluded-items` | - | Glob patterns to exclude (e.g. `*/tmp*`) |
| `-x` | `--allowed-extensions` | (all) | Only scan these extensions. Macros: `IMAGE`, `VIDEO`, `MUSIC`, `TEXT` |
| `-P` | `--excluded-extensions` | - | Skip files with these extensions |
| `-f` | `--file-to-save` | - | Save results to a human-readable text file |
| `-C` | `--compact-file-to-save` | - | Save results as compact (minified) JSON |
| `-p` | `--pretty-file-to-save` | - | Save results as pretty-printed JSON |
| `-R` | `--not-recursive` | false | Scan top-level directory only (no recursion) |
| `-X` | `--exclude-other-filesystems` | false | Skip files on other filesystems (Linux/macOS) |
| `-T` | `--thread-number` | 0 (all) | Limit CPU threads; 0 = all available |
| `-N` | `--do-not-print-results` | false | Suppress result output to console |
| `-M` | `--do-not-print-messages` | false | Suppress all messages, warnings, errors |
| `-W` | `--ignore-error-code-on-found` | false | Return exit code 0 even when files found |
| `-H` | `--disable-cache` | false | Disable the hash/metadata cache entirely |

### Grouped delete flags

Used by tools that produce groups (dup, image, video, music):

| Flag | Long | Default | Description |
|------|------|---------|-------------|
| `-D` | `--delete-method` | NONE | Deletion strategy (see codes below) |
| `-Q` | `--dry-run` | false | Preview operations without executing |
| `-y` | `--move-to-trash` | false | Move to system trash instead of permanent delete |

**Delete method codes:**

| Code | Keeps | Deletes |
|------|-------|---------|
| `AEN` | Newest file | All others |
| `AEO` | Oldest file | All others |
| `AEB` | Biggest file | All others |
| `AES` | Smallest file | All others |
| `ON` | Newest only | Everything else |
| `OO` | Oldest only | Everything else |
| `OB` | Biggest only | Everything else |
| `OS` | Smallest only | Everything else |
| `HARD` | (all, linked) | Replaces duplicates with hard links |
| `NONE` | (all) | Nothing - results only, default |

### Simple delete flags

Used by tools that produce flat lists (empty-folders, empty-files, temp, symlinks, broken):

| Flag | Long | Default | Description |
|------|------|---------|-------------|
| `-D` | `--delete-files` | false | Delete all found items |
| `-Q` | `--dry-run` | false | Preview only |
| `-y` | `--move-to-trash` | false | Move to trash instead |

---

## Output Formats and Exit Codes

All three output formats can be combined in a single command:
- **Text** (`-f results.txt`) - human-readable, one entry per line with group headers
- **Compact JSON** (`-C results.json`) - minified JSON, parse with `jq` or scripts
- **Pretty JSON** (`-p results_pretty.json`) - indented JSON, same structure as compact

**Exit codes:**

| Code | Meaning |
|------|---------|
| `0` | Success, no matching files found (or `-W` flag set) |
| `1` | An error occurred |
| `11` | Success, matching files were found |

Use `-W` / `--ignore-error-code-on-found` in CI/scripts where "files found" is not an error condition.

---

## Tools

### dup - Duplicate Files

```shell
czkawka_cli dup -d <dirs> [options]
```


<img width="600" alt="Screenshot From 2026-06-28 07-19-24" src="https://github.com/user-attachments/assets/138148f4-e360-45d7-9adc-810c6f2b6605" />

| Flag | Default | Description |
|------|---------|-------------|
| `-s` / `--search-method` | `HASH` | `NAME`, `SIZE`, `SIZE_NAME`, `HASH` |
| `-t` / `--hash-type` | `BLAKE3` | `BLAKE3`, `XXH3`, `CRC32` |
| `-m` / `--minimal-file-size` | 8192 | Minimum file size in bytes |
| `-i` / `--maximal-file-size` | (max u64) | Maximum file size in bytes |
| `-u` / `--use-prehash-cache` | false | Cache partial hashes for faster re-scans |
| `-Z` / `--minimal-prehash-cache-file-size` | 257144 | Min size to store in prehash cache |
| `-c` / `--minimal-cached-file-size` | 257144 | Min size to store in hash cache |
| `-l` / `--case-sensitive-name-comparison` | false | Case-sensitive name comparison (NAME method) |
| `-L` / `--allow-hard-links` | false | Treat hard links as separate files |
| `-r` / `--reference-directories` | - | Reference dirs (scanned but never deleted) |
| `-D` / `-Q` / `-y` | NONE/false/false | Delete method / dry-run / move-to-trash |

**Hash method recommendations:**
- Use `HASH` (default) for reliable deduplication.
- Use `NAME` only as a quick experiment - many false positives.
- Use `SIZE` / `SIZE_NAME` only for a rough pre-screen.

Examples:
```shell
# Find duplicates >= 25 bytes in /home/rafal, skip Obrazy dir, IMAGE extensions only
czkawka_cli dup -d /home/rafal -e /home/rafal/Obrazy -m 25 -x IMAGE -f results.txt

# Dry run: show which files would be deleted keeping newest
czkawka_cli dup -d /home/data -D AEN -Q

# Protect /backup, remove duplicates from /incoming, move to trash
czkawka_cli dup -d /incoming -r /backup -D AEN -y
```

---

### empty-folders

```shell
czkawka_cli empty-folders -d <dirs> [options]
```

Finds directories with no files or subdirectories.

| Flag | Default | Description |
|------|---------|-------------|
| `-D` / `-Q` / `-y` | false/false/false | Delete / dry-run / move-to-trash |

Example:
```shell
czkawka_cli empty-folders -d /home/rafal/rr /home/gateway -f results.txt
```

---

### big - Big Files

```shell
czkawka_cli big -d <dirs> [options]
```

| Flag | Default | Description |
|------|---------|-------------|
| `-n` / `--number-of-files` | 50 | Number of files to show |
| `-J` / `--smallest-mode` | false | Find smallest files instead of biggest |
| `-D` / `-Q` / `-y` | | Delete / dry-run / trash |

Example:
```shell
czkawka_cli big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -x VIDEO -f results.txt
```

---

### empty-files

```shell
czkawka_cli empty-files -d <dirs> [options]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--zero-byte-content` | false | Also find files whose entire content is null bytes (`\0`) |
| `--non-printable-content` | false | Also find files with only non-printable ASCII characters (implies `--zero-byte-content`) |
| `-D` / `-Q` / `-y` | | Delete / dry-run / trash |

Example:
```shell
czkawka_cli empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt
```

---

### temp - Temporary Files

```shell
czkawka_cli temp -d <dirs> [options]
```

| Flag | Default | Description |
|------|---------|-------------|
| `-L` / `--extensions` | (built-in list) | Override the list of temporary patterns. If specified, replaces the defaults entirely. Matched using `ends_with` on the lowercased filename. |
| `-D` / `-Q` / `-y` | | Delete / dry-run / trash |

Default patterns: `#`, `thumbs.db`, `.bak`, `~`, `.tmp`, `.temp`, `.ds_store`, `.crdownload`, `.part`, `.cache`, `.dmp`, `.download`, `.partial`

Example:
```shell
czkawka_cli temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D
```

---

### image - Similar Images

```shell
czkawka_cli image -d <dirs> [options]
```

| Flag | Default | Description |
|------|---------|-------------|
| `-s` / `--max-difference` | 5 | Max hash distance (0-40). Lower = stricter. |
| `-g` / `--hash-alg` | `Gradient` | `Mean`, `Gradient`, `Blockhash`, `VertGradient`, `DoubleGradient`, `Median` |
| `-c` / `--hash-size` | 16 | Hash resolution: `8`, `16`, `32`, `64` |
| `-z` / `--image-filter` | `Nearest` | `Lanczos3`, `Nearest`, `Triangle`, `Gaussian`, `CatmullRom` |
| `--geometric-invariance` | `off` | `off`, `mirror-flip`, `mirror-flip-rotate90` |
| `-m` / `--minimal-file-size` | 16384 | Min size in bytes |
| `-i` / `--maximal-file-size` | (max) | Max size in bytes |
| `-J` / `--ignore-same-size` | false | Skip images with identical byte size |
| `-Z` / `--ignore-same-resolution` | false | Skip images with identical pixel dimensions |
| `-L` / `--allow-hard-links` | false | Treat hard links as separate files |
| `-r` / `--reference-directories` | - | Reference directories |
| `-D` / `-Q` / `-y` | | Delete method / dry-run / trash |

**Choosing parameters:**
- Start with `--max-difference 5`, `--hash-size 16`, `--hash-alg Gradient` and adjust from there.
- For high-similarity matching (e.g., thumbnails vs originals), lower `--max-difference` (0-3).
- For matching across compression/watermarks, raise it (10-20 for hash-size 16).
- `--geometric-invariance mirror-flip-rotate90` catches mirrored or rotated copies but is slower.

Example:
```shell
czkawka_cli image -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt
```

---

### video - Similar Videos

Requires `ffmpeg` installed.

```shell
czkawka_cli video -d <dirs> [options]
```

| Flag | Default | Description |
|------|---------|-------------|
| `-t` / `--tolerance` | 10 | Frame difference threshold (0-20) |
| `-A` / `--scan-duration` | 10 | Seconds of video to scan per file |
| `-U` / `--skip-forward-amount` | 15 | Seconds to skip at the start of each video |
| `-B` / `--crop-detect` | true | Detect and ignore letterbox black bars before hashing |
| `--window-count` | (default) | Temporal windows per video (1-20; more = more accurate, slower) |
| `--duration-tolerance-pct` | (default) | Max % duration difference for two videos to be compared |
| `--min-matching-windows` | (default) | Min fraction of matching windows to call videos similar |
| `--subclip-min-match` | (default) | Fraction for subclip detection (a clip inside a longer video) |
| `--check-audio-content` | false | Also compare audio fingerprints (very slow, opt-in) |
| `--generate-thumbnails` | false | Pre-populate thumbnail cache for GUI |
| `-J` / `--ignore-same-size` | false | Ignore groups with same file size |
| `-Z` / `--ignore-same-resolution` | false | Ignore groups with same video resolution |
| `-r` / `--reference-directories` | - | Reference directories |
| `-D` / `-Q` / `-y` | | Delete method / dry-run / trash |

Example:
```shell
czkawka_cli video -d /home/rafal -f results.txt
```

---

### music - Same Music

```shell
czkawka_cli music -d <dirs> [options]
```

| Flag | Default | Description |
|------|---------|-------------|
| `-s` / `--search-method` | `TAGS` | `TAGS` or `CONTENT` |
| `-z` / `--music-similarity` | `track_title,track_artist` | Comma-separated tag fields to compare |
| `-a` / `--approximate-comparison` | false | Allow small differences in tag values |
| `-c` / `--compare-fingerprints-only-with-similar-titles` | false | Speed up CONTENT mode by pre-filtering on title similarity |
| `-l` / `--minimum-segment-duration` | 10.0 | Minimum audio segment length in seconds to compare |
| `-Y` / `--maximum-difference` | 2.0 | Max allowed audio segment difference (0.0-10.0) |
| `-m` / `--minimal-file-size` | 8192 | Min file size in bytes |
| `-r` / `--reference-directories` | - | Reference directories |
| `-D` / `-Q` / `-y` | | Delete method / dry-run / trash |

Available tag fields for `-z`: `track_title`, `track_artist`, `year`, `bitrate`, `genre`, `length`

Example:
```shell
czkawka_cli music -d /home/rafal -e /home/rafal/Pulpit -z "track_artist,year" -f results.txt
```

---

### symlinks - Invalid Symlinks

```shell
czkawka_cli symlinks -d <dirs> [options]
```

Finds symbolic links that point to non-existent targets or form circular chains (>20 jumps).

| Flag | Default | Description |
|------|---------|-------------|
| `-D` / `-Q` / `-y` | false/false/false | Delete / dry-run / trash |

Example:
```shell
czkawka_cli symlinks -d /home/kicikici/ /home/szczek -e /home/kicikici/jestempsem -x jpg -f results.txt
```

---

### broken - Broken Files

```shell
czkawka_cli broken -d <dirs> [options]
```

Finds files that fail to open correctly with their expected library.

| Flag | Default | Description |
|------|---------|-------------|
| `-c` / `--checked-types` | `PDF,AUDIO,IMAGE,ARCHIVE,FONT,MARKUP` | Types to check (comma-separated list) |
| `-D` / `-Q` / `-y` | | Delete / dry-run / trash |

**Available type values:**

| Value | Checks |
|-------|--------|
| `IMAGE` | jpg, jpeg, png, tiff, gif, bmp, ico, webp, exr, avif, others |
| `AUDIO` | mp3, flac, wav, ogg, m4a, aac, others |
| `PDF` | pdf |
| `ARCHIVE` | zip, 7z, gz/tgz, tar, zst, bz2, xz |
| `FONT` | ttf, otf, ttc |
| `MARKUP` | JSON, XML, TOML, YAML, SVG |
| `VIDEO_FFPROBE` | Fast header-only video check (requires ffmpeg) |
| `VIDEO_FFMPEG` | Full decode check - slow, most thorough (requires ffmpeg) |

False positives are possible. Verify files manually before deletion.

Example:
```shell
czkawka_cli broken -d /home/kicikici/ -x jpg -f results.txt
```

---

### ext - Bad Extensions

```shell
czkawka_cli ext -d <dirs> [options]
```

Finds files whose content (detected from magic bytes) does not match their current extension.

| Flag | Default | Description |
|------|---------|-------------|
| `-F` / `--fix-extensions` | false | Rename files to the correct extension automatically |

Example:
```shell
czkawka_cli ext -d /home/czokolada/ -f results.txt
```

---

### bad-names

```shell
czkawka_cli bad-names -d <dirs> [options]
```

At least one check flag must be enabled, otherwise no files will match.

| Flag | Default | Description |
|------|---------|-------------|
| `-u` / `--uppercase-extension` | false | Detect uppercase extensions (e.g. `.JPG`) |
| `-j` / `--emoji-used` | false | Detect emoji characters in filenames |
| `-w` / `--space-at-start-or-end` | false | Detect leading or trailing spaces |
| `-n` / `--non-ascii-graphical` | false | Detect non-ASCII graphical characters |
| `-r` / `--restricted-charset` | - | Allowed special characters; others are flagged (e.g. `_- .`) |
| `-a` / `--remove-duplicated-non-alphanumeric` | false | Detect doubled non-alphanumeric chars (e.g. `file__name`) |
| `-F` / `--fix-names` | false | Rename files automatically to suggested fixed name |
| `-D` / `-Q` / `-y` | | Delete / dry-run / trash |

Example:
```shell
czkawka_cli bad-names -d /home/rafal -u -j -w -n -f results.txt
```

---

### exif-remover

```shell
czkawka_cli exif-remover -d <dirs> [options]
```

Finds image files containing EXIF metadata. Optionally removes tags.

Supported formats: `jpg`, `jpeg`, `jfif`, `png`, `tiff`, `tif`, `avif`, `jxl`, `webp`, `heic`, `heif`.

| Flag | Default | Description |
|------|---------|-------------|
| `-i` / `--ignored-tags` | - | Comma-separated tag names to keep (e.g. `Orientation,DateTime`) |
| `-F` / `--fix-exif` | false | Actually remove EXIF tags |
| `-o` / `--override-file` | false | Overwrite originals (default: write `_cleaned` copy) |

Example:
```shell
czkawka_cli exif-remover -d /home/rafal -x IMAGE -F -f results.txt
```

---

### video-optimizer

```shell
czkawka_cli video-optimizer -d <dirs> transcode [options]
czkawka_cli video-optimizer -d <dirs> crop [options]
```

Requires **ffmpeg** at runtime when using `-F` to actually process videos.

#### transcode

Identifies videos using inefficient codecs.

| Flag | Default | Description |
|------|---------|-------------|
| `-c` / `--excluded-codecs` | - | Codecs to skip/exclude (e.g. `h265,av1,vp9`) |
| `--target-codec` | `h265` | Target codec when `-F` is set: `h264`, `h265`, `av1`, `vp9` |
| `--quality` | 23 | Encoding quality (0-51; lower = better quality, larger file) |
| `--noise-reduction` | `none` | `none` or `hqdn3d` (general-purpose denoiser) |
| `--noise-reduction-strength` | 5 | Denoiser strength (1-10) |
| `--custom-ffmpeg-command` | - | Custom ffmpeg arguments; overrides most encoding options |
| `-t` / `--generate-thumbnails` | false | Generate thumbnail cache |
| `-V` / `--thumbnail-percentage` | 10 | Frame position % for thumbnail (1-99) |
| `-F` / `--fix-videos` | false | Actually transcode found videos |
| `--overwrite-original` | false | Replace original files |
| `--fail-if-not-smaller` | false | Skip files where output is not smaller than original |
| `--limit-video-size` | false | Cap video dimensions |
| `--max-width` | 1920 | Max width in pixels (only with `--limit-video-size`) |
| `--max-height` | 1080 | Max height in pixels (only with `--limit-video-size`) |

#### crop

Identifies videos with black bars (letterbox/pillarbox) or static edge content.

| Flag | Default | Description |
|------|---------|-------------|
| `-m` / `--crop-mechanism` | `blackbars` | `blackbars` or `staticcontent` |
| `-k` / `--black-pixel-threshold` | 32 | Pixel value threshold for "black" (0-128; lower = stricter) |
| `-b` / `--black-bar-percentage` | 90 | Min % of black pixels in a row/column to call it a bar |
| `-s` / `--max-samples` | 20 | Max frames to sample (5-1000) |
| `-z` / `--min-crop-size` | 10 | Minimum crop region size in pixels |
| `-t` / `--generate-thumbnails` | false | Generate thumbnail cache |
| `-F` / `--fix-videos` | false | Actually crop the found videos |
| `--overwrite-original` | false | Replace original files |
| `--target-codec` | - | Optionally also transcode during crop |

Examples:
```shell
# Find videos not already in h265 or av1
czkawka_cli video-optimizer -d /home/rafal transcode -c h265,av1 -f results.txt

# Dry run: preview what would be transcoded to h265
czkawka_cli video-optimizer -d /home/rafal transcode -F -Q

# Find videos with black bars
czkawka_cli video-optimizer -d /home/rafal crop -m blackbars -f results.txt
```

---

## Automation Examples

### Nightly duplicate report saved as JSON (cron)

```shell
#!/bin/bash
czkawka_cli dup \
    -d /home/user/Documents /home/user/Downloads \
    -e /home/user/.cache \
    -p /home/user/logs/duplicates_$(date +%Y%m%d).json \
    -N -M -W
```

### CI: fail if any empty files exist in build output

```shell
czkawka_cli empty-files -d ./build_output -N -M
# exit code 11 if empty files found, 0 if none
```

### Interactive cleanup with dry-run confirmation

```shell
#!/bin/bash
dir="${1:?Usage: $0 <directory>}"

echo "=== Dry run: temporary files in $dir ==="
czkawka_cli temp -d "$dir" -Q -D

echo ""
read -p "Proceed with deletion? [y/N] " answer
if [[ "$answer" == "y" ]]; then
    czkawka_cli temp -d "$dir" -D
    echo "Done."
fi
```

### Protect originals, clean up incoming duplicates

```shell
# /archive is reference - never deleted
# /incoming is cleaned - duplicates moved to trash
czkawka_cli dup \
    -d /incoming \
    -r /archive \
    -D AEN \
    -y
```

### Pre-populate video thumbnail cache for Krokiet

```shell
# Run CLI scan so Krokiet gets instant thumbnails next time it opens
czkawka_cli video -d /your/videos --generate-thumbnails -N -M -W
```

### Find images with GPS data and strip it

```shell
# Dry run first - see what has EXIF
czkawka_cli exif-remover -d /home/user/Photos -x IMAGE -f exif_report.txt

# Strip all EXIF tags (keep Orientation), override original files
czkawka_cli exif-remover -d /home/user/Photos -x IMAGE -F -o -i "Orientation"
```
