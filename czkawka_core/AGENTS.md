# czkawka_core – Architecture Guide

## Overview

`czkawka_core` is the shared scanning engine used by all frontends. It has no UI
dependency. Every scanning tool is implemented here; frontends only configure the
tool structs and drive them via the `Search` trait.

`TOOLS_NUMBER = 14` (defined in `src/lib.rs`).

---

## Source Layout

```
czkawka_core/src/
├── lib.rs                         # Crate root; TOOLS_NUMBER constant
├── localizer_core.rs              # Fluent i18n loader (flc! macro)
├── common/
│   ├── mod.rs                     # Shared helpers (format_time, split_path, …)
│   ├── traits.rs                  # Core traits: Search, CommonData, PrintResults, …
│   ├── tool_data.rs               # CommonToolData struct
│   ├── model.rs                   # ToolType, CheckingMethod, FileEntry, DeleteMethod
│   ├── progress_data.rs           # ProgressData, CurrentStage enum
│   ├── progress_stop_handler.rs   # ProgressThreadHandler, check_if_stop_received
│   ├── dir_traversal.rs           # DirTraversalBuilder, DirTraversalResult
│   ├── cache.rs                   # Generic load/save cache (bincode + JSON)
│   ├── cache/
│   │   └── cleaning.rs            # Outdated cache cleanup
│   ├── config_cache_path.rs       # Platform config/cache path lookup
│   ├── directories.rs             # Directories struct (included/excluded/reference)
│   ├── extensions.rs              # Extensions struct (allowed/excluded filtering)
│   ├── items.rs                   # ExcludedItems (glob pattern matching)
│   ├── consts.rs                  # Extension lists: IMAGE_RS_EXTENSIONS, …
│   ├── ffmpeg_utils.rs            # FFmpeg invocation helpers
│   ├── video_utils.rs             # Video metadata extraction
│   ├── image.rs                   # Image loading helpers
│   ├── process_utils.rs           # Child-process helpers
│   ├── logger.rs                  # Logging configuration
│   └── basic_gui_cli.rs           # Types shared with GUI/CLI boundaries
├── tools/
│   ├── mod.rs                     # Re-exports all tool modules
│   ├── duplicate/                 # Hash/name/size duplicate detection
│   ├── empty_files/
│   ├── empty_folder/
│   ├── big_file/
│   ├── similar_images/            # Perceptual hashing (image_hasher)
│   ├── similar_videos/            # Frame-based video similarity
│   ├── same_music/                # Audio tag + chromaprint fingerprint
│   ├── broken_files/              # ZIP/PDF/audio/image validation
│   ├── bad_extensions/            # Extension vs magic-number mismatch
│   ├── bad_names/                 # Naming policy checks
│   ├── invalid_symlinks/
│   ├── temporary/
│   ├── exif_remover/
│   └── video_optimizer/           # Crop detection + FFmpeg transcoding
└── helpers/
    ├── messages.rs                # Messages struct (errors, warnings, info)
    ├── delayed_sender.rs          # Rate-limited progress sender
    ├── audio_checker.rs           # Audio file validation
    ├── ffprobe.rs                 # ffprobe JSON output parsing
    └── debug_timer.rs             # Debug-build timing
```

---

## Tool Module Layout

Each tool lives in its own directory with three files:

| File | Content |
|------|---------|
| `mod.rs` | Tool struct + `Info` struct + `Parameters` struct (if needed) |
| `core.rs` | Internal scanning functions (`check_files_*`, `hash_calculation`, …) |
| `traits.rs` | Trait implementations: `Search`, `CommonData`, `DeletingItems`, `PrintResults` |

Example — `EmptyFiles`:

```
src/tools/empty_files/
├── mod.rs      # pub struct EmptyFiles { common_data, information, empty_files }
├── core.rs     # fn check_files() → WorkContinueStatus
└── traits.rs   # impl Search, CommonData, PrintResults, DeletingItems, AllTraits
```

---

## Core Traits (`src/common/traits.rs`)

```rust
pub trait Search {
    fn search(&mut self, stop_flag: &Arc<AtomicBool>,
              progress_sender: Option<&Sender<ProgressData>>);
}

pub trait CommonData {
    type Info;
    type Parameters;
    fn get_cd(&self) -> &CommonToolData;
    fn get_cd_mut(&mut self) -> &mut CommonToolData;
    fn found_any_items(&self) -> bool;
    // + common setters/getters
}

pub trait PrintResults: CommonData {
    fn write_results<T: Write>(&self, w: &mut T) -> io::Result<()>;
    fn print_results_to_writer<T: Write>(&self, w: &mut T) -> io::Result<()>;
    fn save_results_to_file_as_json(&self, file: &str, pretty: bool) -> io::Result<()>;
}

pub trait DeletingItems {
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>,
                    progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus;
}

pub trait FixingItems {
    type FixParams;
    fn fix_items(&mut self, stop_flag: &Arc<AtomicBool>,
                 progress_sender: Option<&Sender<ProgressData>>, fix_params: Self::FixParams);
}

pub trait ResultEntry {
    fn get_path(&self) -> &Path;
    fn get_modified_date(&self) -> u64;
    fn get_size(&self) -> u64;
}

pub trait AllTraits: DebugPrint + PrintResults + DeletingItems + CommonData + Search {}
```

---

## CommonToolData (`src/common/tool_data.rs`)

```rust
pub struct CommonToolData {
    pub(crate) tool_type: ToolType,
    pub(crate) text_messages: Messages,        // Accumulated warnings / errors
    pub(crate) directories: Directories,       // Included / excluded / reference paths
    pub(crate) extensions: Extensions,         // Allowed / excluded extensions
    pub(crate) excluded_items: ExcludedItems,  // Glob patterns (e.g. "*/.*")
    pub(crate) recursive_search: bool,
    pub(crate) delete_method: DeleteMethod,
    pub(crate) maximal_file_size: u64,
    pub(crate) minimal_file_size: u64,
    pub(crate) stopped_search: bool,
    pub(crate) use_cache: bool,
    pub(crate) delete_outdated_cache: bool,
    pub(crate) save_also_as_json: bool,
    pub(crate) use_reference_folders: bool,
    pub(crate) dry_run: bool,
    pub(crate) move_to_trash: bool,
    pub(crate) hide_hard_links: bool,
}
```

---

## DirTraversal (`src/common/dir_traversal.rs`)

Builder-pattern filesystem traversal used by all tools:

```rust
let result = DirTraversalBuilder::new()
    .common_data(&self.common_data)
    .group_by(|fe: &FileEntry| fe.size)   // Groups entries by this key
    .stop_flag(stop_flag)
    .progress_sender(progress_sender)
    .checking_method(CheckingMethod::Size)
    .build()
    .run();

match result {
    DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => { … }
    DirTraversalResult::Stopped => return WorkContinueStatus::Stop,
}
```

**Internals:**
- Rayon `into_par_iter().with_max_len(2)` for parallel folder processing.
- Two-phase: visit root dirs/files first, then recurse batch by batch.
- Applies extension, exclusion, and size filters during traversal.
- On Unix: optional filesystem-boundary checking (`exclude_other_filesystems`).
- Progress tracked via `ProgressThreadHandler` (dedicated thread, 200 ms interval).

---

## Progress Reporting (`src/common/progress_stop_handler.rs`)

```rust
pub struct ProgressData {
    pub sstage: CurrentStage,       // Current operation
    pub current_stage_idx: u8,      // Index of current stage
    pub max_stage_idx: u8,          // Max stages for this tool
    pub entries_checked: usize,
    pub entries_to_check: usize,
    pub bytes_checked: u64,
    pub bytes_to_check: u64,
    pub tool_type: ToolType,
}
```

`ProgressThreadHandler` spawns a background thread that polls `AtomicUsize`/`AtomicU64`
counters and sends `ProgressData` to the frontend channel every ~200 ms.

Call `handler.increase_items(n)` / `handler.increase_size(n)` from rayon tasks
(using `Relaxed` ordering — fast, no synchronization overhead).

---

## Cancellation

```rust
// In hot paths:
if check_if_stop_received(stop_flag) {
    return WorkContinueStatus::Stop;  // or break / return None in rayon
}
```

`stop_flag` is `Arc<AtomicBool>` with `Relaxed` ordering — sufficient for simple
"should I stop?" polling without synchronization cost. The frontend sets it to
`true`; the backend polls and stops gracefully.

---

## Cache (`src/common/cache.rs`)

```rust
// Save
save_cache_to_file_generalized::<T>(
    cache_file_name,    // e.g. "cache_duplicates.bin"
    &btree_map,         // BTreeMap<String, T> (String = canonical path)
    save_also_as_json,
    minimum_file_size,
);

// Load
let (messages, opt_cache) =
    load_cache_from_file_generalized_by_path::<T>(
        cache_file_name,
        delete_outdated_cache,
        &used_files,    // Current BTreeMap to validate against
    );
```

- Serialized with `bincode` (binary). Optionally also as `.json`.
- Cached entries validated by path + size + mtime on load.
- 8 GB memory limit on serialization.
- Each tool has its own version constant (`CACHE_DUPLICATE_VERSION`, …).

---

## Rayon Usage Patterns

```rust
// Directory traversal
folders.into_par_iter().with_max_len(2).map(|dir| { … }).while_some().collect();

// Deletion with early exit
items.into_par_iter().map(|e| {
    if check_if_stop_received(stop_flag) { return None; }
    // … process e …
    Some(result)
}).while_some().flatten().collect()
```

Thread count is globally controlled via `set_number_of_threads(n)` which stores
to a `LazyLock<Mutex<Option<usize>>>` and is applied via rayon's thread pool.

---

## ToolType Enum (`src/common/model.rs`)

```rust
pub enum ToolType {
    Duplicate, EmptyFolders, EmptyFiles, InvalidSymlinks,
    BrokenFiles, BadExtensions, BadNames, BigFile, SameMusic,
    SimilarImages, SimilarVideos, TemporaryFiles, ExifRemover,
    VideoOptimizer,
    #[default]
    None,
}
```

Tools that support reference directories: `Duplicate`, `SameMusic`,
`SimilarImages`, `SimilarVideos` (`may_use_reference_paths()`).

---

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `rayon` | Parallel iterators |
| `crossbeam-channel` | Progress channels |
| `blake3` | Fast file hashing |
| `image` + `image_hasher` | Image loading + perceptual hash |
| `lofty` | Audio tag reading |
| `symphonia` | Audio decoding |
| `rusty-chromaprint` | Audio fingerprinting |
| `vid_dup_finder_lib` | Video similarity |
| `bincode` | Cache serialization |
| `zip` | ZIP validation |
| `i18n-embed` + `rust-embed` | Fluent translations |
| `trash` | Move-to-trash |
| `directories-next` | Config/cache path |
| `fun_time` | `#[fun_time]` timing attribute |

Optional (behind features):
- `heif` → `libheif-rs` – HEIC/HEIF image support
- `libraw` → `rawler` / `libraw-rs` – RAW photo support
- `libavif` – AVIF image support
- `xdg_portal_trash` – FlatPak trash via XDG portal
