# Czkawka Core - Library Guide

`czkawka_core` is a pure Rust library that implements all scanning tools. It has no UI dependency and is designed to be embedded in any frontend.

## Table of Contents

- [Tools](#tools)
  - [Duplicate Finder](#duplicate-finder)
  - [Empty Files](#empty-files)
  - [Empty Folders](#empty-folders)
  - [Big Files](#big-files)
  - [Temporary Files](#temporary-files)
  - [Invalid Symlinks](#invalid-symlinks)
  - [Same Music](#same-music)
  - [Similar Images](#similar-images)
  - [Similar Videos](#similar-videos)
  - [Broken Files](#broken-files)
  - [Bad Extensions](#bad-extensions)
  - [Bad Names](#bad-names)
  - [Exif Remover](#exif-remover)
  - [Video Optimizer](#video-optimizer)
- [Cache](#cache)
- [Adding as Dependency](#adding-as-dependency)
- [Integration Pattern](#integration-pattern)
  - [Common configuration (all tools)](#common-configuration-all-tools)
  - [Running a scan](#running-a-scan)
  - [Progress reporting](#progress-reporting)
  - [Stop flag](#stop-flag)
  - [Reading results](#reading-results)
  - [Deleting files](#deleting-files)
- [Config and Cache Paths](#config-and-cache-paths)

---

## Tools

### Duplicate Finder

Finds files with identical content (or matching name/size, depending on the method).

**How it works**: Files are first grouped by size (any unique-sized file cannot have a duplicate). Within each size group, a prehash of the first and last 4 KB is computed - files with unique prehashes are eliminated. Finally, the remaining files are fully hashed and grouped by hash.

**Check methods:**
- **Name** - groups files sharing the same filename. Very fast but produces many false positives.
- **Size** - groups files of exactly the same byte size. Fast but unreliable; almost always produces false positives.
- **Size + Name** - groups by both size and name simultaneously. Still unreliable.
- **Hash** - cryptographic hash of the full file content. Slowest but practically 100% reliable.

**Integration:**

```rust
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::common::model::{CheckingMethod, HashType};

let params = DuplicateFinderParameters::new(
    CheckingMethod::Hash,
    HashType::Blake3,
    false,   // use_prehash_cache
    257_144, // minimal_cache_file_size (bytes)
    257_144, // minimal_prehash_cache_file_size (bytes)
    false,   // case_sensitive_name_comparison
);
let mut finder = DuplicateFinder::new(params);
// configure with CommonData setters, then:
finder.search(&stop_flag, progress_sender);
```

**Results:**

```rust
// Hash mode (most common)
// BTreeMap<file_size: u64, Vec<group: Vec<DuplicateEntry>>>
for (size, groups) in finder.get_files_sorted_by_hash() {
    for group in groups {
        for entry in group {
            // entry.path, entry.size, entry.modified_date, entry.hash
        }
    }
}

// With reference folders:
// BTreeMap<size, Vec<(reference: DuplicateEntry, duplicates: Vec<DuplicateEntry>)>>
for (size, pairs) in finder.get_files_with_identical_hashes_referenced() { }

// Name mode:   finder.get_files_sorted_by_names()  -> BTreeMap<String, Vec<DuplicateEntry>>
// Size mode:   finder.get_files_sorted_by_size()   -> BTreeMap<u64,    Vec<DuplicateEntry>>
// SizeName:    finder.get_files_sorted_by_size_name()
```

`DuplicateEntry`: `path: PathBuf`, `size: u64`, `modified_date: u64`, `hash: String`.

**Info struct** (after search): `finder.get_information()` returns `Info` with `number_of_groups_by_hash`, `number_of_duplicated_files_by_hash`, `lost_space_by_hash`, `scanning_time`, etc.

**Hash types:** `HashType::Blake3` (recommended, cryptographic), `HashType::Xxh3` (fastest, non-cryptographic), `HashType::Crc32`.

---

### Empty Files

Finds files that contain no meaningful data.

**How it works**: Scans all files and checks metadata size. Optional content modes inspect actual bytes.

**Modes:**
- **Basic** (default) - files with size exactly 0 bytes.
- **Zero byte content** - also includes non-empty files whose entire content is null bytes (`\0`). Useful for finding placeholder files that consume disk space but carry no data.
- **Non-printable content** - also includes files containing only non-printable ASCII: `\0`, `\t`, `\n`, `\v`, `\f`, `\r`, space. Implies zero byte content.

**Integration:**

```rust
use czkawka_core::tools::empty_files::{EmptyFiles, EmptyFilesParameters};
use czkawka_core::common::model::CheckingMethod;

let params = EmptyFilesParameters::new(
    CheckingMethod::EmptyFilesContent, // or CheckingMethod::None for basic mode
);
let mut tool = EmptyFiles::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<FileEntry>
for entry in tool.get_empty_files() { }
```

---

### Empty Folders

Finds directories that contain no files or subdirectories.

**How it works**: Creates an entry for each directory and marks it as potentially empty. As directory contents are examined, any directory with a file or non-empty subdirectory is recursively un-marked as empty (propagated upward to all parents). Directories still marked as empty after the full traversal are reported.

**Integration:**

```rust
use czkawka_core::tools::empty_folder::EmptyFolder;

let mut tool = EmptyFolder::new();
tool.search(&stop_flag, progress_sender);

// Results: HashMap<PathBuf, FolderEntry>
for (path, entry) in tool.get_empty_folder_list() { }
```

`FolderEntry` contains the path and folder metadata.

---

### Big Files

Finds the N largest (or N smallest) files in the scanned directories.

**Integration:**

```rust
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};

let params = BigFileParameters::new(50, SearchMode::BiggestFiles); // or SmallestFiles
let mut tool = BigFile::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<FileEntry>, already sorted largest-first (or smallest-first)
for entry in tool.get_big_files() { }
```

---

### Temporary Files

Finds files matching a predefined list of temporary file patterns.

**How it works**: Checks each file's lowercased name against a set of extensions and suffixes. The default list includes: `.tmp`, `.bak`, `.part`, `.crdownload`, `.temp`, `.cache`, `.dmp`, `.download`, `.partial`, `thumbs.db`, `~`, `#`. The list can be replaced entirely by providing a custom list.

**Integration:**

```rust
use czkawka_core::tools::temporary::{Temporary, TemporaryParameters};

let params = TemporaryParameters::new(
    Vec::new(),  // empty = use built-in defaults; provide strings to override entirely
);
let mut tool = Temporary::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<FileEntry>
for entry in tool.get_temporary_files() { }
```

---

### Invalid Symlinks

Finds symbolic links whose target does not exist or that form a circular chain.

**How it works**: For each symlink, the tool checks whether its target path exists. If it does not, the symlink is flagged. A chain that exceeds 20 hops is flagged as a circular reference.

**Integration:**

```rust
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;

let mut tool = InvalidSymlinks::new();
tool.search(&stop_flag, progress_sender);

// Results: Vec<SymlinkEntry>
for entry in tool.get_invalid_symlinks() { }
```

`SymlinkEntry` contains `path`, `destination_path`, and `type_of_error` (`InfiniteRecursion` or `NonExistentFile`).

---

### Same Music

Finds duplicate or similar music files.

**How it works:**

**Tags mode** - Reads metadata tags from each file. User selects which tag fields to compare (artist, title, album, year, bitrate, genre, length). Tags are normalized (lowercase, non-alphanumeric stripped, optional approximate comparison). Only files with non-empty tags in the selected fields are compared.

**Content mode** - Computes an audio fingerprint for each file. Optionally pre-filters by title similarity to reduce pairwise comparisons. Compares fingerprints using a configurable similarity threshold and minimum matching segment length.

**Integration:**

```rust
use czkawka_core::tools::same_music::{SameMusic, SameMusicParameters, MusicSimilarity};
use czkawka_core::common::model::CheckingMethod;

let params = SameMusicParameters::new(
    MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST, // which fields to compare
    false,                       // approximate_comparison
    CheckingMethod::AudioTags,   // AudioTags or AudioContent
    10.0,                        // minimum_segment_duration (seconds, for AudioContent mode)
    2.0,                         // maximum_difference (0.0-10.0, for AudioContent mode)
    false,                       // compare_fingerprints_only_with_similar_titles
);
let mut tool = SameMusic::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<Vec<MusicEntry>>
for group in tool.get_similar_music_entries() { }

// With reference folders: Vec<(MusicEntry, Vec<MusicEntry>)>
for (reference, duplicates) in tool.get_similar_music_referenced() { }
```

`MusicEntry` contains `path`, `size`, `modified_date`, and tag fields (`track_title`, `track_artist`, `year`, `bitrate`, `genre`, `length`).

`MusicSimilarity` is a bitflag - combine fields with `|`: `TRACK_TITLE | TRACK_ARTIST | YEAR | BITRATE | GENRE | LENGTH`.

---

### Similar Images

Finds images that look alike but are not byte-for-byte identical (different resolution, watermarks, compression re-saves, etc.).

**How it works:**
1. Images are collected by extension.
2. Previously computed hashes are loaded from cache.
3. Each uncached image is resized to the chosen hash size (8×8, 16×16, 32×32, or 64×64 pixels) using the chosen resize filter, then a perceptual hash is computed.
4. Hashes are stored in a BK-tree for efficient Hamming-distance search.
5. All hashes are compared; pairs whose Hamming distance is below `max_difference` are grouped as similar.

Unlike cryptographic hashes, perceptual hashes are designed so visually similar images produce similar (close) hashes.

**Geometric invariance** - when enabled, additional hashes are computed for mirrored, flipped, and/or rotated (90-degree) variants of each image. This allows matching flipped/rotated copies at the cost of more hashing.

**Hash algorithms:** `Gradient` (default in CLI, good for most photos), `Mean` (default in Krokiet), `Blockhash` (does not resize), `VertGradient`, `DoubleGradient`, `Median`.

**Resize filters:** `Nearest` (fastest, default in CLI), `Lanczos3` (best quality, default in Krokiet), `Triangle`, `Gaussian`, `CatmullRom`.

**SIMILAR_VALUES** constant - preset `max_difference` values per hash size and similarity level:

```rust
use czkawka_core::tools::similar_images::SIMILAR_VALUES;
// SIMILAR_VALUES[hash_size_idx][preset_idx]
// hash_size_idx: 0=8, 1=16, 2=32, 3=64
// preset_idx: 0=Original..5=Minimal
let recommended = SIMILAR_VALUES[1][2]; // hash_size 16, Medium preset
```

**Integration:**

```rust
use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters, GeometricInvariance};
use czkawka_core::re_exported::{HashAlg, FilterType};

let params = SimilarImagesParameters::new(
    10,                         // max_difference (0-40)
    16,                         // hash_size (8, 16, 32 or 64)
    HashAlg::Gradient,
    FilterType::Nearest,
    false,                      // exclude_images_with_same_size
    false,                      // exclude_images_with_same_resolution
    GeometricInvariance::Off,   // Off | MirrorFlip | MirrorFlipRotate90
);
let mut tool = SimilarImages::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<Vec<ImagesEntry>>
for group in tool.get_similar_images() { }

// With reference folders: Vec<(ImagesEntry, Vec<ImagesEntry>)>
for (reference, similars) in tool.get_similar_images_referenced() { }
```

`ImagesEntry`: `path`, `size`, `modified_date`, `width: u32`, `height: u32`, `hashes: Vec<Vec<u8>>`, `difference: u32`.

---

### Similar Videos

Finds visually similar videos using perceptual frame hashing. Requires **ffmpeg** at runtime.

**How it works**: Videos are grouped by duration (within a configurable tolerance). For each video, several frames are extracted at regular intervals (temporal windows) and perceptual hashes are computed. Two videos are considered similar if a sufficient fraction of their window hashes match within the given tolerance. Subclip detection identifies cases where a shorter video is contained within a longer one.

An optional audio mode computes audio fingerprints and compares them independently of the visual match.

**Integration:**

```rust
use czkawka_core::tools::similar_videos::{
    SimilarVideos, SimilarVideosParameters,
    DEFAULT_WINDOW_COUNT, DEFAULT_DURATION_TOLERANCE_PCT,
    DEFAULT_MIN_MATCHING_WINDOWS, DEFAULT_SUBCLIP_MIN_MATCH,
    DEFAULT_SKIP_FORWARD_AMOUNT, DEFAULT_CROP_DETECT,
    DEFAULT_VIDEO_PERCENTAGE_FOR_THUMBNAIL, DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE,
    DEFAULT_AUDIO_SIMILARITY_PERCENT, DEFAULT_AUDIO_LENGTH_RATIO,
    DEFAULT_AUDIO_MAXIMUM_DIFFERENCE, DEFAULT_AUDIO_MIN_DURATION_SECONDS,
};

let params = SimilarVideosParameters::new(
    10,                                    // tolerance (0-20)
    false,                                 // exclude_videos_with_same_size
    false,                                 // exclude_videos_with_same_resolution
    DEFAULT_SKIP_FORWARD_AMOUNT,           // seconds to skip at video start
    10,                                    // scan duration per video (seconds)
    DEFAULT_CROP_DETECT,                   // detect/remove letterbox bars before hashing
    DEFAULT_WINDOW_COUNT,                  // temporal windows per video
    DEFAULT_DURATION_TOLERANCE_PCT,        // max duration difference %
    DEFAULT_MIN_MATCHING_WINDOWS,          // min fraction of windows to match
    DEFAULT_SUBCLIP_MIN_MATCH,             // min fraction for subclip detection
    false,                                 // generate_thumbnails
    DEFAULT_VIDEO_PERCENTAGE_FOR_THUMBNAIL,// thumbnail position %
    false,                                 // generate_thumbnail_grid_instead_of_single
    DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE, // tiles per side in grid thumbnail
    false,                                 // check_audio_content (set true for audio comparison)
    DEFAULT_AUDIO_SIMILARITY_PERCENT,      // min % of matching audio
    DEFAULT_AUDIO_MAXIMUM_DIFFERENCE,      // max audio segment difference
    DEFAULT_AUDIO_LENGTH_RATIO,            // min ratio shorter/longer audio
    DEFAULT_AUDIO_MIN_DURATION_SECONDS,    // min audio duration for comparison
);
let mut tool = SimilarVideos::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<Vec<VideosEntry>>
for group in tool.get_similar_videos() { }

// With reference folders: Vec<(VideosEntry, Vec<VideosEntry>)>
for (reference, similars) in tool.get_similar_videos_referenced() { }
```

`VideosEntry`: `path`, `size`, `modified_date`, `duration: f64`.

---

### Broken Files

Finds files that fail to open or validate with their expected library.

**How it works**: Files are collected by extension. Each file is opened and parsed by the appropriate library. If parsing returns an error, the file is flagged as broken. Some errors that commonly produce false positives are suppressed.

**Supported checkers:**

| Checker | File types | Default |
|---------|-----------|---------|
| `IMAGE` | jpg, jpeg, png, tiff, gif, bmp, ico, webp, exr, avif, and others | on |
| `AUDIO` | mp3, flac, wav, ogg, m4a, aac, and others | on |
| `PDF` | pdf | on |
| `ARCHIVE` | zip, 7z, gz/tgz, tar, zst, bz2, xz | on |
| `FONT` | ttf, otf, ttc | on |
| `MARKUP` | JSON, XML, TOML, YAML, SVG | on |
| `VIDEO_FFPROBE` | mp4, mkv, avi, mov, webm, and others - fast headers-only check | off (requires ffmpeg) |
| `VIDEO_FFMPEG` | same as above - slow full decode | off (requires ffmpeg) |

**Integration:**

```rust
use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};

let checked_types = CheckedTypes::IMAGE | CheckedTypes::AUDIO | CheckedTypes::PDF
    | CheckedTypes::ARCHIVE | CheckedTypes::FONT | CheckedTypes::MARKUP;
// Add video types if ffmpeg is available:
// | CheckedTypes::VIDEO_FFPROBE | CheckedTypes::VIDEO_FFMPEG

let params = BrokenFilesParameters::new(checked_types);
let mut tool = BrokenFiles::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<BrokenEntry>
for entry in tool.get_broken_files() {
    // entry.path, entry.size, entry.modified_date
    // entry.get_error_string() -> human-readable reason
}
```

Note: false positives can occur. Verify files manually before deletion.

---

### Bad Extensions

Finds files whose actual content (detected from magic bytes) does not match their current extension.

**How it works**: Reads the first few bytes of each file and matches them against known magic byte signatures to determine the likely file type. The detected MIME type is expanded to all valid extensions for that type. If the file's current extension is not among them, the file is flagged.

**Integration:**

```rust
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};

let params = BadExtensionsParameters::new();
let mut tool = BadExtensions::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<BadFileEntry>
for entry in tool.get_bad_extensions_files() {
    // entry.path - current path
    // entry.proper_extensions - suggested extension(s)
}
```

Use `FixingItems` to automatically rename files:

```rust
use czkawka_core::common::traits::FixingItems;
tool.fix_items(&stop_flag, progress_sender, ());
```

---

### Bad Names

Finds files with problematic names that may cause issues on certain filesystems or operating systems.

**How it works**: Each filename is checked against the enabled rules. Any rule that matches adds the file to results. A suggested corrected name is computed for each flagged file.

**Checks (each independently configurable):**
- **Uppercase extension** - e.g. `photo.JPG` instead of `photo.jpg`
- **Emoji** - emoji characters in the filename
- **Spaces at start or end** - leading or trailing whitespace
- **Non-ASCII graphical** - characters outside the ASCII printable range
- **Restricted charset** - characters not in the user-defined allowed set (e.g. only `a-z`, `0-9`, `_`, `-`, `.`)
- **Duplicated non-alphanumeric** - repeated special characters (e.g. `file__name`, `doc---v2`)

**Integration:**

```rust
use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters};

let params = BadNamesParameters::new(
    true,         // uppercase_extension
    true,         // emoji_used
    true,         // space_at_start_or_end
    true,         // non_ascii_graphical
    None,         // restricted_charset: Option<String> (e.g. Some("_- .".to_string()))
    false,        // remove_duplicated_non_alphanumeric
);
let mut tool = BadNames::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<BadFileEntry>
for entry in tool.get_bad_names() {
    // entry.path - current path
    // entry.proper_name - suggested fixed name
}
```

Use `FixingItems` to rename automatically:

```rust
tool.fix_items(&stop_flag, progress_sender, ());
```

---

### Exif Remover

Finds image files containing EXIF metadata (GPS, camera model, creation timestamps, etc.).

**How it works**: Reads EXIF tags from each image using `little_exif`. Files with any non-ignored tags are reported. A cache stores which files have already been scanned (keyed by path, size, and modification date).

**Supported formats**: jpg, jpeg, jfif, png, tiff, tif, avif, jxl, webp, heic, heif.

**Integration:**

```rust
use czkawka_core::tools::exif_remover::{ExifRemover, ExifRemoverParameters};

let params = ExifRemoverParameters::new(
    vec!["Orientation".to_string(), "ColorSpace".to_string()], // tags to ignore (keep)
);
let mut tool = ExifRemover::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<ExifEntry>
for entry in tool.get_exif_files() {
    // entry.path, entry.size, entry.modified_date
    // entry.exif_tags: Vec<ExifTag>  (only tags not in the ignored list)
}
```

Use `FixingItems` to strip tags:

```rust
use czkawka_core::common::traits::FixingItems;
// FixParams: (override_original: bool)
tool.fix_items(&stop_flag, progress_sender, false); // false = write _cleaned copy
tool.fix_items(&stop_flag, progress_sender, true);  // true  = overwrite original
```

---

### Video Optimizer

Identifies videos that are candidates for optimization. Two modes:

**Transcode mode** - finds videos not already using efficient codecs. The user supplies a list of "excluded" codecs (e.g. `["h265", "av1"]`); any video using a codec not in that list is flagged as a candidate for transcoding. With `-F`/`fix_items`, the tool actually re-encodes the files using ffmpeg.

**Crop mode** - analyzes video frames to detect black bars (letterbox/pillarbox) or static edge content. Reports the optimal crop rectangle for each video. With `-F`/`fix_items`, the tool applies the crop with ffmpeg.

Optional features: noise reduction (`hqdn3d`), hardware-accelerated encoding, custom ffmpeg command override, thumbnail generation.

**Integration:**

```rust
use czkawka_core::tools::video_optimizer::{VideoOptimizer, VideoOptimizerParameters, VideoOptimizerMode};

let params = VideoOptimizerParameters::new(VideoOptimizerMode::Transcode {
    excluded_codecs: vec!["h265".to_string(), "av1".to_string()],
    generate_thumbnails: false,
    thumbnail_percentage: 10,
    generate_thumbnail_grid: false,
    thumbnail_grid_tiles_per_side: 3,
});
let mut tool = VideoOptimizer::new(params);
tool.search(&stop_flag, progress_sender);

// Results: Vec<VideoEntry>
for entry in tool.get_video_optimizer_entries() {
    // entry.path, entry.size, entry.codec, entry.width, entry.height
}
```

---

## Cache

Cache files are stored in the shared czkawka cache directory. All frontends (CLI, Krokiet, GTK) read and write the same cache.

Notable files:
- `cache_duplicates_<HASH>.bin/.json` - per-hash-type duplicate cache
- `cache_similar_image_<SIZE>_<HASH>_<FILTER>_<INVARIANCE>.bin/.json` - perceptual image hashes (one file per unique parameter set)
- `cache_similar_videos.bin/.json` - video signatures
- `cache_music_tags.bin/.json`, `cache_music_fingerprints.bin/.json` - music metadata/fingerprints
- `cache_exif.bin/.json` - EXIF scan results
- `cache_broken_files.txt` - broken file check results

By default `.bin` (binary/bincode format) is loaded; if the `.bin` file is missing the `.json` fallback is used. Call `set_save_also_as_json(true)` to write both formats.

JSON files can be manually edited (e.g., to update paths when moving a collection to another machine).

---

## Adding as Dependency

```toml
# Cargo.toml
[dependencies]
czkawka_core = { path = "../czkawka_core" }
# OR from git:
# czkawka_core = { git = "https://github.com/qarmin/czkawka" }
```

Optional features (require native libraries installed at build and runtime):

| Feature | Library | Purpose |
|---------|---------|---------|
| `heif` | `libheif` | HEIF/HEIC image support |
| `libraw` | `libraw` | RAW camera image support |
| `libavif` | `libavif`, `libdav1d` | AVIF image support |

The `similar_videos` tool requires **ffmpeg** at runtime only (not a build dependency).

---

## Integration Pattern

All tools follow the same pattern: create with parameters, configure with `CommonData` trait setters, call `.search()`, then read results.

### Common configuration (all tools)

Every tool struct implements `CommonData`, which exposes setters shared across all tools:

```rust
use czkawka_core::common::tool_data::CommonData;

// Required: directories to scan
tool.set_included_paths(vec![PathBuf::from("/home/user/Documents")]);

// Optional filters
tool.set_excluded_paths(vec![PathBuf::from("/home/user/.cache")]);
tool.set_excluded_items(vec!["*/tmp*".to_string(), "*/.git".to_string()]);
tool.set_allowed_extensions(vec!["jpg".to_string(), "png".to_string()]);
tool.set_excluded_extensions(vec!["db".to_string()]);

// File size limits (in bytes)
tool.set_minimal_file_size(8_192);
tool.set_maximal_file_size(u64::MAX);

// Traversal
tool.set_recursive_search(true);          // default: true
tool.set_exclude_other_filesystems(false); // Unix only

// Cache behaviour
tool.set_use_cache(true);                  // default: true
tool.set_delete_outdated_cache(true);      // default: true
tool.set_save_also_as_json(false);         // write .json alongside .bin cache

// Reference paths (supported by: Duplicate, SimilarImages, SimilarVideos, SameMusic)
tool.set_reference_paths(vec![PathBuf::from("/home/user/Archive")]);
tool.set_use_reference_folders(true);

// Deletion (used when calling delete_files() after search)
use czkawka_core::common::tool_data::DeleteMethod;
tool.set_delete_method(DeleteMethod::AllExceptNewest);
tool.set_dry_run(false);
tool.set_move_to_trash(false);
```

`DeleteMethod` values: `None`, `Delete`, `AllExceptNewest`, `AllExceptOldest`, `OneNewest`, `OneOldest`, `AllExceptBiggest`, `AllExceptSmallest`, `OneBiggest`, `OneSmallest`, `HardLink`.

### Running a scan

All tools implement the `Search` trait:

```rust
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use czkawka_core::common::traits::Search;

let stop_flag = Arc::new(AtomicBool::new(false));

// Blocking call - returns when scan is complete or stopped
tool.search(&stop_flag, None);               // no progress reporting
tool.search(&stop_flag, Some(&progress_tx)); // with progress channel
```

### Progress reporting

Progress is sent over a `crossbeam_channel::Sender<ProgressData>`:

```rust
use crossbeam_channel::unbounded;
use czkawka_core::common::progress_data::{ProgressData, CurrentStage};

let (progress_tx, progress_rx) = unbounded::<ProgressData>();

// Spawn receiver before calling search
std::thread::spawn(move || {
    for progress in progress_rx {
        println!(
            "[{:?}] stage {}/{}: {}/{} entries",
            progress.sstage,
            progress.current_stage_idx,
            progress.max_stage_idx,
            progress.entries_checked,
            progress.entries_to_check,
        );
        // Use progress.bytes_checked / progress.bytes_to_check for byte-level progress
        // Use CurrentStage::check_if_loading_saving_cache() to detect indeterminate phases
    }
});

tool.search(&stop_flag, Some(&progress_tx));
```

`ProgressData` fields:
- `sstage: CurrentStage` - which sub-step is running (e.g. `DuplicateFullHashing`, `SimilarImagesCalculatingHashes`)
- `current_stage_idx / max_stage_idx: u8` - overall progress through tool stages
- `entries_checked / entries_to_check: usize` - item-level progress within the current stage
- `bytes_checked / bytes_to_check: u64` - byte-level progress (set for hashing stages)
- `tool_type: ToolType` - which tool is running
- `checking_method: CheckingMethod` - which mode is active

`CurrentStage::check_if_loading_saving_cache()` returns `true` during cache load/save phases, where `entries_to_check` is 0 and no progress bar can be shown (use an indeterminate indicator instead).

### Stop flag

To interrupt a running scan from another thread:

```rust
use std::sync::atomic::Ordering;

stop_flag.store(true, Ordering::Relaxed);
```

The tool checks the flag regularly and returns from `.search()` early. Check `tool.get_stopped_search()` afterwards to know if the scan was interrupted.

### Reading results

Each tool exposes its own getter methods (described per-tool above). After a search, also check messages:

```rust
let messages = tool.get_text_messages();
// messages.errors   - Vec<String>
// messages.warnings - Vec<String>
// messages.messages - Vec<String>
```

### Deleting files

After a successful search, call `delete_files()` to apply the configured `DeleteMethod`:

```rust
use czkawka_core::common::traits::DeletingItems;

tool.set_delete_method(DeleteMethod::AllExceptNewest);
tool.delete_files(&stop_flag, Some(&progress_tx));
```

For tools that support fixing (renaming, EXIF removal, transcoding), use the `FixingItems` trait:

```rust
use czkawka_core::common::traits::FixingItems;
tool.fix_items(&stop_flag, Some(&progress_tx), fix_params);
```

---

## Config and Cache Paths

Paths depend on the `cache_name` and `config_name` strings passed to `set_config_cache_path`. Existing frontends use:

| Frontend | `cache_name` | `config_name` |
|----------|-------------|---------------|
| Krokiet | `"Czkawka"` | `"Krokiet"` |
| GTK / CLI | `"Czkawka"` | `"Czkawka"` |
| Cedinia | `"cedinia"` | `"cedinia"` |

Resulting paths (Linux example, using `directories_next::ProjectDirs`):
- Cache `"Czkawka"` → `~/.cache/czkawka/`
- Config `"Krokiet"` → `~/.config/krokiet/`
- Config `"Czkawka"` → `~/.config/czkawka/`

`set_config_cache_path` **must be called once before any cache or config access**:

```rust
use czkawka_core::common::config_cache_path::{set_config_cache_path, get_config_cache_path};

// Call once at startup - choose names appropriate for your app
let result = set_config_cache_path("MyApp", "MyApp");
// result.infos / result.warnings contain diagnostic messages

if let Some(paths) = get_config_cache_path() {
    println!("Cache: {}", paths.cache_folder.display());
    println!("Config: {}", paths.config_folder.display());
}
```

Override with environment variables (checked before the default path):

```shell
CZKAWKA_CONFIG_PATH="/custom/config" CZKAWKA_CACHE_PATH="/custom/cache" ./my_app
```
