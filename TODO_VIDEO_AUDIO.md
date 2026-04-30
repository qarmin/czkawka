# TODO: Video Audio Fingerprinting

> **Status: implemented** as a sub-mode of `SimilarVideos`.
> The items below document what was done and what still remains (GUI wiring, etc.).


## Overview

The existing `SameMusic` tool (`czkawka_core/src/tools/same_music/`) already:
- decodes audio via **Symphonia**,
- generates Chromaprint fingerprints via **rusty-chromaprint**,
- compares fingerprints with `match_fingerprints`.

This feature reuses that pipeline but applies it to video files, adding a new step: extract the
audio stream from a video container before fingerprinting.

---

## New tool / extension options

Two implementation paths exist:

### Option A – Extend `SimilarVideos` with an audio-fingerprint mode
Add a new `CheckingMethod::VideoAudioContent` (or similar) to the existing `SimilarVideos` tool.
The tool would optionally compute an audio fingerprint alongside the visual hash and expose a
separate comparison pass.

### Option B – Separate `SameVideoAudio` tool (preferred)
Mirror the structure of `SameMusic` but scan video file extensions.
This keeps the two responsibilities orthogonal and avoids bloating `SimilarVideos`.

The rest of this document assumes **Option B**.

---

## Implementation steps

### 1. Audio extraction from video containers

Symphonia can already demux many video containers (MKV, MP4, WebM, AVI, …) and decode the
embedded audio track – the same `calc_fingerprint_helper` function used in `same_music/core.rs`
works without modification as long as we point it at a video file and ask it to ignore video
tracks.

The relevant change in `calc_fingerprint_helper`:
```rust
// Current code looks for the first non-null codec.
// For video files we need to find specifically an AUDIO track.
let track = format
    .tracks()
    .iter()
    .find(|t| t.codec_params.codec != CODEC_TYPE_NULL
           && t.codec_params.channels.is_some()) // audio tracks have channels
    .ok_or_else(|| "no supported audio track".to_string())?;
```

If no audio track is present the file is silently skipped (not an error).

Supported containers depend on which Symphonia feature flags are enabled; add the
`symphonia-bundle-mp3`, `symphonia-format-mkv`, `symphonia-format-mp4`, etc. features as needed
in `czkawka_core/Cargo.toml`.

> **No native dependencies needed.** Symphonia is pure Rust: no ffmpeg, no libav.

### 2. New `VideoAudioEntry` struct

```rust
// czkawka_core/src/tools/same_video_audio/mod.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoAudioEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub duration_seconds: u32,   // needed for duration-group filtering (see §4)
    pub fingerprint: Vec<u32>,
}
```

`duration_seconds` is extracted from Symphonia's `track.codec_params.time_base` +
`track.codec_params.n_frames`, or from the container's `FormatReader::metadata()` if available.

### 3. New `SameVideoAudio` tool

Mirrors `SameMusic` in structure:

```
czkawka_core/src/tools/same_video_audio/
    mod.rs      – structs, parameters, constants
    core.rs     – check_files, calculate_fingerprint, check_for_duplicate_fingerprints
    traits.rs   – PrintResults (CSV / JSON)
    tests.rs
```

Scanning steps (same pattern as `SameMusic`):
1. `check_files` – directory traversal filtered to video extensions.
2. `calculate_fingerprint` – extract audio stream, compute Chromaprint fingerprint, cache result.
3. `check_for_duplicate_fingerprints` – compare fingerprints within groups.

Cache file name example: `cache_same_video_audio_fingerprints_{CACHE_VERSION}.bin`

### 4. Duration-group filtering (key new feature)

To avoid comparing a 1-minute clip against a 3-hour film, videos are grouped by duration before
fingerprint comparison.  Only files whose durations are within a configurable tolerance of each
other are ever compared.

#### Parameter

```rust
pub struct SameVideoAudioParameters {
    // ... existing fingerprint params (same as SameMusicParameters) ...
    pub minimum_segment_duration: f32,
    pub maximum_difference: f64,

    /// Maximum allowed relative difference in audio duration between two videos
    /// before they are excluded from comparison.
    ///
    /// Value is a fraction in [0.0, 1.0]:
    ///   0.0  = only compare videos of identical duration
    ///   0.2  = allow up to 20 % difference  (default)
    ///   1.0  = disable grouping, compare everything with everything
    pub max_duration_difference_ratio: f64,
}
```

Default: `0.20` (20 %).

#### Allowed range

`0.0 ..= 1.0`.  Validated with `assert!` in `SameVideoAudioParameters::new`.

#### Grouping algorithm

```
given: entries: Vec<VideoAudioEntry>, max_ratio: f64

1. Sort entries ascending by duration_seconds.
2. Greedy sweep: start a new group with the first unassigned entry as the
   "anchor" duration D_anchor.
   - Add the next entry if its duration <= D_anchor * (1 + max_ratio).
   - When an entry exceeds that threshold, close the current group
     and start a new one with the current entry as anchor.
3. Discard groups with < 2 entries.
4. Run fingerprint comparison within each group independently.
```

This is O(n log n) (dominated by the sort) and produces non-overlapping groups, which is
acceptable because a short video and a long video truly cannot carry the same audio content.

Alternative: for each pair compute the ratio and only compare if within threshold – O(n²), avoid
for large collections.

#### CLI exposure

```
--max-duration-diff-ratio <RATIO>   [default: 0.20]
    Compare audio only between videos whose durations differ by at most RATIO (0.0–1.0).
    Set to 1.0 to compare all videos regardless of length.
```

#### GUI exposure (krokiet / cedinia)

Add a numeric spin-box (0–100, unit %) mapped to the `[0.0, 1.0]` float parameter.
Store in `Settings` global alongside the other `SameMusic` fingerprint settings.

### 5. Caching

Same cache infrastructure as `SameMusic`:
- `load_and_split_cache_generalized_by_path` / `save_and_connect_cache_generalized_by_path`
- Invalidate when `CACHE_VERSION` changes.
- Cache key: absolute path + modified date.

`duration_seconds` and `fingerprint` are both cached; no re-extraction needed on subsequent runs.

### 6. Supported video extensions

Start with the set already used by `SimilarVideos`:
`mp4`, `mkv`, `avi`, `mov`, `wmv`, `flv`, `webm`, `m4v`, `mpg`, `mpeg`, `ts`, `3gp`

Filter in `CommonToolData` / `DirTraversal` the same way other tools specify their extensions.

### 7. Progress stages

Add new `CurrentStage` variants (in `czkawka_core/src/common/progress_data.rs`):
```rust
SameVideoAudioCacheLoadingFingerprints,
SameVideoAudioCalculatingFingerprints,
SameVideoAudioCacheSavingFingerprints,
SameVideoAudioComparingFingerprints,
```

### 8. Frontend integration (krokiet)

- Add `ActiveTool::SameVideoAudio` Slint enum variant.
- Add column index constants in `krokiet/src/common.rs` (path, duration, size, …).
- Add `connect_same_video_audio.rs` wiring the scan callbacks.
- Add UI panel in `krokiet/ui/` (can reuse the `SameMusic` panel as template).
- Add translation keys to `krokiet/i18n/en/krokiet.ftl`.

### 9. CLI integration (czkawka_cli)

Add `same-video-audio` subcommand via `clap` derive, exposing:
- All standard common options (dirs, excluded dirs, extensions, …)
- `--min-segment-duration`
- `--max-difference`
- `--max-duration-diff-ratio`

---

## Non-goals / out of scope

- Extracting video frames or doing visual comparison (that is `SimilarVideos`).
- Transcoding or re-encoding audio before fingerprinting.
- Using ffmpeg or any native library – Symphonia handles all demuxing in pure Rust.
- Comparing audio across different durations groups when `max_duration_difference_ratio = 1.0`
  is not set (too expensive and rarely useful).

---

## Open questions

- Should `max_duration_difference_ratio = 1.0` (disabled grouping) warn the user about potential
  performance impact?
- Should the tool also optionally match by audio tags (title, artist) extracted from the video's
  embedded metadata, as a pre-filter (same as `compare_fingerprints_only_with_similar_titles`)?
- Is it worth sharing the `calc_fingerprint_helper` function by moving it to a common module
  rather than duplicating it?

